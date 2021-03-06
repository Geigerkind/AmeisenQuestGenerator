use mysql::Pool;
use mysql::prelude::{BinQuery, WithParams};

use crate::value_objects::Position;
use std::fs::File;
use std::io::Write;

#[derive(Debug)]
pub enum QuestHolder {
    Npc {
        id: u32,
        position: Position,
    },
    Gameobject {
        id: u32,
        position: Position,
    },
    Item {
        id: u32
    },
    None
}

impl QuestHolder {
    pub fn new(pool: &Pool, quest_id: u32, is_starter: bool) -> Self {
        let quest_npc_query = format!("SELECT A.id, B.position_x, B.position_y, B.position_z FROM {} A JOIN creature B ON A.id = B.id WHERE A.quest=:quest_id LIMIT 1",
                                      if is_starter { "creature_queststarter" } else { "creature_questender" });
        let creature_info: Option<(u32, f64, f64, f64)> = quest_npc_query.with(params! { "quest_id" => quest_id }).first(pool).unwrap();
        if let Some((npc_id, x, y, z)) = creature_info {
            return QuestHolder::Npc {
                id: npc_id,
                position: Position { x, y, z },
            };
        }

        if is_starter {
            let item_info: Option<u32> = "SELECT StartItem FROM quest_template WHERE ID=:start_quest LIMIT 1".with(params! { "start_quest" => quest_id }).first(pool).unwrap();
            if let Some(item_id) = item_info {
                return QuestHolder::Item { id: item_id };
            }
        }

        let quest_gameobject_query = format!("SELECT A.id, B.position_x, B.position_y, B.position_z FROM {} A JOIN gameobject B ON A.id = B.id WHERE A.quest=:quest_id LIMIT 1",
                                             if is_starter { "gameobject_queststarter" } else { "gameobject_questender" });
        let gameobject_info: Option<(u32, f64, f64, f64)> = quest_gameobject_query.with(params! { "quest_id" => quest_id }).first(pool).unwrap();
        if let Some((gameobject_id, x, y, z)) = gameobject_info {
            return QuestHolder::Gameobject {
                id: gameobject_id,
                position: Position { x, y, z },
            };
        }
        unreachable!()
    }

    pub fn export(&self, file: &mut File) {
        if let QuestHolder::Npc { id, position } = &self {
            let _ = file.write_all(format!("                () => (wowInterface.ObjectManager.GetClosestWowUnitByNpcId(new List<int> {{ {} }}), new Vector3({:.2}f, {:.2}f, {:.2}f)),\n", id, position.x, position.y, position.z).as_bytes());
        } else {
            unimplemented!()
        }
    }
}