use dbtest1::establish_connection;
use dbtest1::push_t1;
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

    push_t1(connection, &title);
    println!("new data add '{}' ", title );
    
    let cnt = getcount_t1(connection );

    let all = getall_t1(connection);

    for one in all {
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
