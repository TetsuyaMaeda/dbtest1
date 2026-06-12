
use diesel::prelude::*;


#[derive(Queryable,Debug)]
pub struct ST1 {
    pub id:i32,
    pub name:String,
}



// insert するときに使う一時構造体の定義。なぜ別構造体が 必要なの？
use crate::schema::T1;
#[derive(Insertable)]
#[diesel(table_name=T1)]
pub struct NewT1 <'a> {
    pub name:&'a str,
}
/// 有効期限がある、str参照をメンバーに含む構造体。




pub struct ST2 {
    pub id:i32,
    pub name:String,
}


pub struct ST3 {
    pub id:i32,
    pub name:String,
}



