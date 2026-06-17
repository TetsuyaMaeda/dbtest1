use dbtest1::establish_connection;
use dbtest1::insert_t1;
use dbtest1::getcount_t1;
use dbtest1::getall_t1;
use dbtest1::updatename_t1;
pub mod models;
pub mod schema;

use dbtest1::models::ST1;

fn main()
 {
    println!("Hello, world!");

    let connection = &mut establish_connection();


    let title="a3cde".to_string();

    let ans = insert_t1(connection, &title);
    match (ans) {
        Ok(v) => println! ("insert return value={} ",v),
        Err(a) => println! ( "insert return Error={} , a "),
    }

    println!("new data add '{}' ", title );
    
    let cnt = getcount_t1(connection );
    println!("Total data cnt ={}", cnt );

    
    let all = getall_t1(connection);
    let s = match (all) {
        Ok(t) => t,
        Err(a) => vec!{},
    };

    for one in s {
        match one.name {
            Some(val) =>    println! ("{}={}",one.id, val),
            None =>         println! ("{}={}",one.id, "NULL"),
        }

    }
    
    // 更新
    let onedata = ST1 {
        id: 3,
        name: Some (String::from ("buhibuhi")),
    };


    let result = updatename_t1(connection, onedata);

}
