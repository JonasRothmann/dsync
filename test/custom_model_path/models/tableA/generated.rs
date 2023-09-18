/* @generated and managed by dsync */

use crate::diesel::*;
use crate::schema::*;
use diesel::QueryResult;
use serde::{Deserialize, Serialize};


type ConnectionType = diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, Selectable)]
#[diesel(table_name=tableA, primary_key(_id))]
pub struct TableA {
    pub _id: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable)]
#[diesel(table_name=tableA)]
pub struct CreateTableA {
    pub _id: i32,
}



    pub fn create(db: &mut ConnectionType, item: &CreateTableA) -> QueryResult<Self> {
        use crate::schema::tableA::dsl::*;

        insert_into(tableA).values(item).get_result::<Self>(db)
    }

    pub fn read(db: &mut ConnectionType, param__id: i32) -> QueryResult<Self> {
        use crate::schema::tableA::dsl::*;

        tableA.filter(_id.eq(param__id)).first::<Self>(db)
    }

    pub fn delete(db: &mut ConnectionType, param__id: i32) -> QueryResult<usize> {
        use crate::schema::tableA::dsl::*;

        diesel::delete(tableA.filter(_id.eq(param__id))).execute(db)
    }

}