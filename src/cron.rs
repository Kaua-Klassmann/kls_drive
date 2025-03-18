use std::sync::Arc;

use chrono::Duration;
use entity::user;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, sqlx::types::chrono::Utc};
use tokio::task;
use tokio_cron_scheduler::{Job, JobScheduler};

use crate::{connections, services};

pub async fn init_cron_jobs() {
    let scheduler = JobScheduler::new().await.unwrap();

    let job = Job::new_async("0 0 0 * * *", |_, _| {
        Box::pin(async move {
            delete_all_users_with_activator_generation_date_greater_than_1_days().await;
        })
    })
    .unwrap();

    scheduler.add(job).await.unwrap();
    scheduler.start().await.unwrap();
}

async fn delete_all_users_with_activator_generation_date_greater_than_1_days() {
    let db = connections::database::get_db_connection().await;
    let redis = Arc::new(connections::redis::get_redis_connection().await);

    let one_day_ago = (Utc::now() - Duration::days(1)).date_naive();

    let users_result = user::Entity::find()
        .filter(user::Column::ActivatorGenerationDate.lt(one_day_ago))
        .all(db)
        .await;

    if users_result.is_err() {
        return;
    }

    let users = users_result.unwrap();

    if users.is_empty() {
        return;
    }

    let _ = user::Entity::delete_many()
        .filter(user::Column::ActivatorGenerationDate.lt(one_day_ago))
        .exec(db)
        .await;

    for user in users {
        let redis_clone = Arc::clone(&redis);

        task::spawn(async move {
            let mut redis = &mut redis_clone.get().await.unwrap();

            let _ = services::redis::delete_user(&mut redis, &user.email).await;
            let _ = services::redis::del_activate_user(&mut redis, &user.activation.unwrap()).await;
        });
    }
}
