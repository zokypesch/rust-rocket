use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
// mod schema;
// use schema::heroes;

table! {
    heroes {
        id -> Nullable<Integer>,
        name -> Varchar,
        identity -> Varchar,
        hometown -> Varchar,
        age -> Integer,
    }
}

#[table_name = "heroes"]
#[derive(AsChangeset, Serialize, Deserialize, Queryable, Insertable)]
pub struct Hero {
    pub id: Option<i32>,
    pub name: String,
    pub identity: String,
    pub hometown: String,
    pub age: i32
}

impl Hero {
    pub fn create(hero: Hero, connection: &MysqlConnection) -> Hero {
        diesel::insert_into(heroes::table)
            .values(&hero)
            .execute(connection)
            .expect("Error creating new hero");

        
        // let users = sql_query("SELECT * FROM heroes ORDER BY id")
        //     .load(&connection);
        
        heroes::table.order(heroes::id.desc()).first(connection).unwrap()
        // let id : Option<i32> = Some(30);
        // let new_hero = Hero{id: id, name: String::from("udin"), identity: String::from("123123123"), hometown: String::from("jakarta"), age: 30};

        // new_hero
    }

    pub fn read(connection: &MysqlConnection) -> Vec<Hero> {
        heroes::table.order(heroes::id).load::<Hero>(connection).unwrap()
    }

    pub fn update(id: i32, hero: Hero, connection: &MysqlConnection) -> bool {
        diesel::update(heroes::table.find(id)).set(&hero).execute(connection).is_ok()
    }

    pub fn delete(id: i32, connection: &MysqlConnection) -> bool {
        diesel::delete(heroes::table.find(id)).execute(connection).is_ok()
    }
}
