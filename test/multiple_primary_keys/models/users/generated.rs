/* @generated and managed by dsync */

use crate::diesel::*;
use crate::schema::*;
use diesel::QueryResult;
use serde::{Deserialize, Serialize};


type ConnectionType = diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset, Selectable)]
#[diesel(table_name=users, primary_key(name,address))]
pub struct User {
    pub name: String,
    pub address: String,
    pub secret: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=users)]
pub struct CreateUser {
    pub name: String,
    pub address: String,
    pub secret: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset, Default)]
#[diesel(table_name=users)]
pub struct UpdateUser {
    pub secret: Option<String>,
}


    pub fn create(db: &mut ConnectionType, item: &CreateUser) -> QueryResult<Self> {
        use crate::schema::users::dsl::*;

        insert_into(users).values(item).get_result::<Self>(db)
    }

    pub fn read(db: &mut ConnectionType, param_name: String, param_address: String) -> QueryResult<Self> {
        use crate::schema::users::dsl::*;

        users.filter(name.eq(param_name)).filter(address.eq(param_address)).first::<Self>(db)
    }

    pub fn delete(db: &mut ConnectionType, param_name: String, param_address: String) -> QueryResult<usize> {
        use crate::schema::users::dsl::*;

        diesel::delete(users.filter(name.eq(param_name)).filter(address.eq(param_address))).execute(db)
    }

}