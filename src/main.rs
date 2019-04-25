#![feature(plugin)]
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;
use rocket_contrib::json;

#[macro_use] extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;

extern crate redis;
use redis::Commands;

// use db & schema
mod db;
mod schema;

mod hero;
use hero::{Hero};

// struct Examp {
//     pub id: Option<i32>,
//     pub name: Option<String>,
//     pub identity: Option<String>,
//     pub hometown: Option<String>,
//     pub age: Option<i32>
// }

// #![feature(core_intrinsics)]
// fn print_type_of<T>(_: &T) -> () {
//     let type_name =
//         unsafe {
//             (*std::intrinsics::get_tydesc::<T>()).name
//         };
//     println!("{}", type_name);
// }

fn fetch_redis() -> i32 {
// fn fetch_redis() -> String {
    // connect to redis
    let client = redis::Client::open("redis://redis-mail-stg.statefulset.svc.cluster.local:6379/").unwrap();
    let con = client.get_connection().unwrap();
    // throw away the result, just make sure it does not fail
    // let _ : () = con.set("my_key", 42)?;
    // read back the key and return it.  Because the return value
    // from the function is a result for integer this will automatically
    // convert into one.
    // let map : HashMap<String, i32> = con.hget("et_my_prizes@MURID15556")?;
    // let count : i32 = con.get("my_counter");
    // count.to_string()
    // let some : String = con.get("my_counter").unwrap_or(String);
    // some
    // con.get("asep-test")
    let count : i32 = con.get("my_counter").unwrap();
    count
}

#[get("/")]
fn read_redis() -> JsonValue {
    // let exam : redis::RedisResult<String> = fetch_redis();
    let test = fetch_redis();

    // exam.get_string();
    // print_type_of(&exam);
    // let client = redis::Client::open("redis://redis-mail-stg.statefulset.svc.cluster.local:6379/")?;
    // let con = client.get_connection()?;
    // let _strs = exam::String
    // let x : str = con.get("asep-test")?;
    json!({
        "test": test,
    })
}



#[post("/", data = "<hero>", format = "application/json")]
fn create(hero: Json<Hero>, connection: db::Connection) -> Json<Hero> {
// fn create(hero: JsonValue, connection: db::Connection) -> JsonValue {
    let insert = Hero { id: None, ..hero.into_inner() };
    // println!("Result {}", insert.age);
    Json(Hero::create(insert, &connection))
    // json!({"success": false})
}

// #[post("/", data = "<examp>", format = "application/json")]
// fn hello(examp: Json<Examp>) -> JsonValue {
//     let insert = Examp{ id: None, ..examp.into_inner() };
    // json!({"success": false})
    // match form_value {
    //     Some(x) => {
    //         json!({"test": x })
    //     },
    //     None => {
    //         json!({"test": "test"})
    //     }
    // }
//     json!({"test": insert})
    
// }

#[get("/")]
fn read(connection: db::Connection) -> JsonValue {
    json!(Hero::read(&connection))
}

#[put("/<id>", data = "<hero>")]
fn update(id: i32, hero: Json<Hero>, connection: db::Connection) -> JsonValue {
    let update = Hero { id: Some(id), ..hero.into_inner() };
    json!({
        "success": Hero::update(id, update, &connection)
    })
    
}

#[delete("/<id>")]
fn delete(id: i32, connection: db::Connection) -> JsonValue {
    json!({
        "success": Hero::delete(id, &connection)
    })
}

fn main() {
    rocket::ignite()
        // .manage(db::connect())
        // .mount("/hero", routes![create, update, delete])
        // .mount("/heroes", routes![read])
        .mount("/hello", routes![read_redis])
        .launch();
}