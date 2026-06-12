use dbtest1::establish_connection;
use dbtest1::push_t1;
use dbtest1::getcount_t1;

fn main() {
    println!("Hello, world!");

    let connection = &mut establish_connection();

    let title="abcde".to_string();

    push_t1(connection, &title);
    println!("new data add '{}' ", title );

    let cnt = getcount_t1(connection );
    println!("record cnt = '{}' ", cnt );
}
