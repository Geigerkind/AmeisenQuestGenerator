extern crate change_case;
extern crate chull;
extern crate dbscan;
#[macro_use]
extern crate mysql;

use mysql::Pool;

use crate::entities::Quest;

pub mod entities;
pub mod value_objects;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let quest_id = u32::from_str_radix(&args.get(1).expect("First argument should be a quest id!"), 10).expect("First argument should be a quest id!");
    let pool = Pool::new("mysql://root:vagrant@localhost:33306/world").unwrap();
    let quest = Quest::new(&pool, quest_id);
    println!("{:?}", quest);
    quest.export();
}
