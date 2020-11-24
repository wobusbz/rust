use mysql::prelude::*;
use mysql::*;

fn main() {
    let url = "mysql://root:123@localhost:3306/zshq";
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();
    let ret: Option<i32> = conn
        .query_first("SELECT `id` FROM ht_user ORDER BY id DESC LIMIT 1")
        .unwrap();

    println!("Hello, world! {:?}", Some(ret));
    let name = String::from("wuahurou");
    println!("{}", name);
}
