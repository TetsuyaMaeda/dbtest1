use diesel::{mysql::MysqlConnection, result::Error::InvalidCString};
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
use diesel::result::Error;
pub fn insert_t1(conn: &mut MysqlConnection, name: &str ) -> Result<usize,Error>  {
    use crate::schema::T1::dsl::{id, T1};

    let onedata = NewT1 { name };

    let ans = diesel::insert_into(T1)
        .values(&onedata)
        .execute(conn);
        
    ans

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

pub fn getall_t1(conn: &mut MysqlConnection ) -> Result< Vec<ST1>, Error>  {

        let result  = T1::table
        .filter(T1::id.gt(0))
        .load::<ST1>(conn);

        result


}

pub fn get_t1 (conn: &mut MysqlConnection,onedata:ST1 )-> Result<Vec<ST1> , Error> {
     let result  = T1::table
     .filter(T1::id.eq (onedata.id))
     .load::<ST1>(conn);
     result

}

pub fn updatename_t1(conn: &mut MysqlConnection,onedata:ST1 )-> Result<usize , Error> {
    let res =  diesel::update (schema::T1::dsl::T1.find(onedata.id))
    .set(T1::name.eq(onedata.name.clone()))
    .execute(conn);

    res
} 
pub fn update_insert_t1 ( conn:&mut MysqlConnection,onedata:ST1 ) -> Result<usize , Error> {

    // onedata を updatename_t1() にパラメータ渡しすると、もう使えなくなるのでその前にコピー。
    let optionname = onedata.name.clone();
    let doupdate = updatename_t1( conn,onedata ) ;
    match doupdate {
        Ok(_v) => {
            if _v > 0 {return doupdate}
        }, 
        Err(_err) => {  },
    }        

    // Err または update のAns=0のときはここに流れてくる。
    // iniert するための文字列を、onedata.nameから取り出す。
    let paramv = 
        match optionname {
            None => "".to_string(), 
            Some(v) =>  v.clone(),
        }
    ;
    let ans_insert = insert_t1 ( conn,&paramv);
    ans_insert


}

pub fn delete_t1(conn: &mut MysqlConnection,onedata:ST1 )-> usize {
     let res =  diesel::delete(schema::T1::dsl::T1.find(onedata.id))
    .execute(conn)
    .expect("Error updating users");
    
    res
}



