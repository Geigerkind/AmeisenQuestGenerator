extern crate chull;
extern crate dbscan;
#[macro_use]
extern crate mysql;

use mysql::Pool;
use crate::entities::Quest;

pub mod entities;
pub mod value_objects;

fn main() {
    let pool = Pool::new("mysql://root:vagrant@localhost:3306/world_wotlk").unwrap();
    let quest = Quest::new(&pool, 4402);
    println!("{:?}", quest);
}
