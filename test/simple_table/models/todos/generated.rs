/* @generated and managed by dsync */

use crate::diesel::*;
use crate::schema::*;
use diesel::QueryResult;
use serde::{Deserialize, Serialize};


type ConnectionType = diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset, Selectable)]
#[diesel(table_name=todos, primary_key(id))]
pub struct Todo {
    pub id: i32,
    pub unsigned: u32,
    pub unsigned_nullable: Option<u32>,
    pub text: String,
    pub completed: bool,
    pub type_: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=todos)]
pub struct CreateTodo {
    pub unsigned: u32,
    pub unsigned_nullable: Option<u32>,
    pub text: String,
    pub completed: bool,
    pub type_: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset, Default)]
#[diesel(table_name=todos)]
pub struct UpdateTodo {
    pub unsigned: Option<u32>,
    pub unsigned_nullable: Option<Option<u32>>,
    pub text: Option<String>,
    pub completed: Option<bool>,
    pub type_: Option<String>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}


    pub fn create(db: &mut ConnectionType, item: &CreateTodo) -> QueryResult<Self> {
        use crate::schema::todos::dsl::*;

        insert_into(todos).values(item).get_result::<Self>(db)
    }

    pub fn read(db: &mut ConnectionType, param_id: i32) -> QueryResult<Self> {
        use crate::schema::todos::dsl::*;

        todos.filter(id.eq(param_id)).first::<Self>(db)
    }

    pub fn delete(db: &mut ConnectionType, param_id: i32) -> QueryResult<usize> {
        use crate::schema::todos::dsl::*;

        diesel::delete(todos.filter(id.eq(param_id))).execute(db)
    }

}