use diesel::mysql::MysqlConnection;
use diesel::prelude::*;


use dotenvy::dotenv;
use std::env;

pub mod models;
pub mod schema;


pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}


use crate::schema::T1;

use self::models::{NewT1, ST1};

pub fn push_t1(conn: &mut MysqlConnection, name: &str )  /*->ST1*/  {
    use crate::schema::T1::dsl::{id, T1};

    let onedata = NewT1 { name };

    diesel::insert_into(T1)
        .values(&onedata)
        .execute(conn)
        .expect("Error saving new task");
    /*    
    let ans :ST1  = T1.order(id.desc()).first(conn).unwrap();
    ans
     */
}

pub fn getcount_t1(conn: &mut MysqlConnection ) ->i64 {

    let mut count_all=0;

    let resans =  T1::table
    .filter(T1::id.gt(0))
    .count()
    .get_result(conn);
    match resans {
        Ok(v)=>  count_all=v,
        Err(error) =>  count_all=0,
    };

    count_all
}

pub fn getall_t1(conn: &mut MysqlConnection ) -> Vec<ST1> {

        let result  = T1::table
        .filter(T1::id.gt(0))
        .load::<ST1>(conn)
        .expect("Error loading tasks");
    
        result
}

pub fn updatename_t1(conn: &mut MysqlConnection,onedata:ST1 )-> usize {
    let res =  diesel::update (schema::T1::dsl::T1.find(onedata.id))
    .set(T1::name.eq(onedata.name.clone()))
    .execute(conn)
    .expect("Error updating users");
    
    res
    

}
/*
pub fn getonet_t1(conn: &mut MysqlConnection  , paramid:i32)  ->ST1 {

}
 */
