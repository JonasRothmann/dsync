/* @generated and managed by dsync */

use crate::diesel::*;
use crate::schema::*;
use diesel::QueryResult;
use serde::{Deserialize, Serialize};
use crate::data::models::table_a::TableA;

type ConnectionType = diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset, Identifiable, Associations, Selectable)]
#[diesel(table_name=tableB, primary_key(_id), belongs_to(TableA, foreign_key=link))]
pub struct TableB {
    pub _id: i32,
    pub link: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=tableB)]
pub struct CreateTableB {
    pub _id: i32,
    pub link: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset, Default)]
#[diesel(table_name=tableB)]
pub struct UpdateTableB {
    pub link: Option<i32>,
}


    pub fn create(db: &mut ConnectionType, item: &CreateTableB) -> QueryResult<Self> {
        use crate::schema::tableB::dsl::*;

        insert_into(tableB).values(item).get_result::<Self>(db)
    }

    pub fn read(db: &mut ConnectionType, param__id: i32) -> QueryResult<Self> {
        use crate::schema::tableB::dsl::*;

        tableB.filter(_id.eq(param__id)).first::<Self>(db)
    }

    pub fn delete(db: &mut ConnectionType, param__id: i32) -> QueryResult<usize> {
        use crate::schema::tableB::dsl::*;

        diesel::delete(tableB.filter(_id.eq(param__id))).execute(db)
    }

}