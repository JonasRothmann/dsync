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
    pub created_at: chrono::NaiveDateTime,
}


#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset, Default)]
#[diesel(table_name=todos)]
pub struct UpdateTodo {
    pub created_at: Option<chrono::NaiveDateTime>,
}


#[derive(Debug, Serialize)]
pub struct CursorPaginationResult<T, K> {
    pub items: Vec<T>,
    pub start_cursor: Option<K>,
    pub end_cursor: Option<K>,
    pub has_previous_page: bool,
    pub has_next_page: bool,
}

impl Todo {

    pub fn create(db: &mut ConnectionType) -> QueryResult<Self> {
        use crate::schema::todos::dsl::*;

        insert_into(todos).default_values().get_result::<Self>(db)
    }

    pub fn read(db: &mut ConnectionType, param_id: i32) -> QueryResult<Self> {
        use crate::schema::todos::dsl::*;

        todos.filter(id.eq(param_id)).first::<Self>(db)
    }

    pub fn delete(db: &mut ConnectionType, param_id: i32) -> QueryResult<usize> {
        use crate::schema::todos::dsl::*;

        diesel::delete(todos.filter(id.eq(param_id))).execute(db)
    }

    
    /// Paginates through the table based on a cursor
    pub fn paginate_cursor(db: &mut ConnectionType, limit: i64, cursor: i32) -> QueryResult<CursorPaginationResult<Self, i32>> {
        use crate::schema::todos::dsl::*;

        let limit = if limit < 1 { 1 } else { limit };
        let items = todos.filter(id.eq(param_id)).limit(limit).load::<Self>(db)?;

        let start_cursor = items.first().map(|it| i32);
        let end_cursor = items.last().map(|it| i32);

        let has_previous_page = start_cursor.is_some();
        let has_next_page = end_cursor.is_some();

        Ok(CursorPaginationResult {
            items,
            start_cursor,
            end_cursor,
            has_previous_page,
            has_next_page,
        })
    }

}
pub struct TodoBatcher {
    db: ConnectionType
}

pub type TodoLoader = Loader<i32, Todo, TodoBatcher>;

impl TodoBatcher {
    pub fn new(db: &mut ConnectionType) -> TodoLoader {
        Loader::new(Self { db })
    }
}

#[async_trait]
impl BatchFn<i32, Todo> for TodoBatcher {
    async fn load(&self, keys: &[i32]) -> Result<HashMap<i32, Todo>, Error> {
        let mut items = Todo::read_many(&mut self.db, keys).await?;
        let mut map = HashMap::new();

        for item in items.drain(..) {
            map.insert(item.id, item);
        }

        Ok(map)
    }
}
        