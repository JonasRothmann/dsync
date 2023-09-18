/* @generated and managed by dsync */

use crate::diesel::*;
use crate::schema::*;
use diesel::QueryResult;
use serde::{Deserialize, Serialize};


type ConnectionType = diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset, Selectable)]
#[diesel(table_name=fang_tasks, primary_key(id))]
pub struct FangTask {
    pub id: uuid::Uuid,
    pub metadata: serde_json::Value,
    pub error_message: Option<String>,
    pub state: crate::schema::sql_types::FangTaskState,
    pub task_type: String,
    pub uniq_hash: Option<String>,
    pub retries: i32,
    pub scheduled_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=fang_tasks)]
pub struct CreateFangTask {
    pub id: uuid::Uuid,
    pub metadata: serde_json::Value,
    pub error_message: Option<String>,
    pub state: crate::schema::sql_types::FangTaskState,
    pub task_type: String,
    pub uniq_hash: Option<String>,
    pub retries: i32,
    pub scheduled_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset, Default)]
#[diesel(table_name=fang_tasks)]
pub struct UpdateFangTask {
    pub metadata: Option<serde_json::Value>,
    pub error_message: Option<Option<String>>,
    pub state: Option<crate::schema::sql_types::FangTaskState>,
    pub task_type: Option<String>,
    pub uniq_hash: Option<Option<String>>,
    pub retries: Option<i32>,
    pub scheduled_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}


    pub fn create(db: &mut ConnectionType, item: &CreateFangTask) -> QueryResult<Self> {
        use crate::schema::fang_tasks::dsl::*;

        insert_into(fang_tasks).values(item).get_result::<Self>(db)
    }

    pub fn read(db: &mut ConnectionType, param_id: uuid::Uuid) -> QueryResult<Self> {
        use crate::schema::fang_tasks::dsl::*;

        fang_tasks.filter(id.eq(param_id)).first::<Self>(db)
    }

    pub fn delete(db: &mut ConnectionType, param_id: uuid::Uuid) -> QueryResult<usize> {
        use crate::schema::fang_tasks::dsl::*;

        diesel::delete(fang_tasks.filter(id.eq(param_id))).execute(db)
    }

}