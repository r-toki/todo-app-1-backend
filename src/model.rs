use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::PgPool;

#[derive(Clone, Debug)]
pub struct NewTask {
    pub description: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct Task {
    pub id: i64,
    pub description: String,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Task {
    pub async fn all(pool: &PgPool) -> Result<Vec<Task>> {
        let tasks = sqlx::query_as!(
            Task,
            r#"
SELECT *
FROM tasks
ORDER BY id
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(tasks)
    }

    pub async fn insert(pool: &PgPool, new_task: NewTask) -> Result<Task> {
        let task = sqlx::query_as!(
            Task,
            r#"
INSERT INTO tasks (description)
VALUES ( $1 )
RETURNING *
            "#,
            new_task.description
        )
        .fetch_one(pool)
        .await?;

        Ok(task)
    }

    pub async fn delete(pool: &PgPool, id: i64) -> Result<Task> {
        let task = sqlx::query_as!(
            Task,
            r#"
DELETE FROM tasks
WHERE id = $1
RETURNING *
            "#,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(task)
    }
}
