use crate::value_objects::Position;
use mysql::Pool;
use mysql::prelude::{WithParams, BinQuery};
use crate::entities::Area;
use std::io::Write;
use std::fs::File;

#[derive(Debug)]
pub enum QuestObjective {
    KillAndLoot {
        npc_ids: Vec<u32>,
        areas: Vec<Area>,
        loot_item: Option<u32>,
        amount: u32
    },
    CollectFromGameobject {
        item_id: u32,
        amount: u32,
        gameobject_ids: Vec<u32>,
        positions: Vec<Position>,
    },
    TalkObjective,
}

impl QuestObjective {
    pub fn try_new_kill_objective(pool: &Pool, quest_id: u32, objective_index: usize, objective_npc_id: u32) -> Option<Self> {
        let amount: u32 = format!("SELECT RequiredNpcOrGoCount{0} FROM quest_template WHERE ID=:id AND RequiredNpcOrGo{0}=:objective_npc_id", objective_index)
            .with(params! { "id" => quest_id, "objective_npc_id" => objective_npc_id }).first(pool).unwrap().expect("This RequiredNpcOrGo must exist");

        let npc_ids: Vec<u32> = "SELECT entry FROM creature_template WHERE KillCredit1=:credit OR KillCredit2=:credit OR entry=:credit"
            .with(params!{ "credit" => objective_npc_id }).map(pool, |entry| entry).unwrap();
        let areas = Area::new(pool, &npc_ids);

        if npc_ids.is_empty() {
            return None;
        }
        Some(QuestObjective::KillAndLoot {
            npc_ids,
            areas,
            loot_item: None,
            amount
        })
    }

    pub fn try_new_collect_objective(pool: &Pool, quest_id: u32, objective_index: usize, item_id: u32) -> Option<Self> {
        let amount: u32 = format!("SELECT RequiredItemCount{0} FROM quest_template WHERE ID=:id AND RequiredItemId{0}=:item_id", objective_index)
            .with(params! { "id" => quest_id, "item_id" => item_id }).first(pool).unwrap().expect("This RequiredItemId must exist");

        // TODO: Sometimes its both!
        // Check if its a KillAndLootObjective
        let loot_npc_ids: Vec<u32> = "SELECT CreatureEntry FROM creature_questitem WHERE ItemId=:item_id"
            .with(params!{ "item_id" => item_id }).map(pool, |entry| entry).unwrap();
        if !loot_npc_ids.is_empty() {
            let areas = Area::new(pool, &loot_npc_ids);
            return Some(QuestObjective::KillAndLoot {
                npc_ids: loot_npc_ids,
                areas,
                loot_item: Some(item_id),
                amount
            });
        } else {
            // Gameobject
            let loot_gameobject_ids: Vec<u32> = "SELECT GameObjectEntry FROM gameobject_questitem WHERE ItemId=:item_id"
                .with(params!{ "item_id" => item_id }).map(pool, |entry| entry).unwrap();
            if !loot_gameobject_ids.is_empty() {
                let mut positions = Vec::new();
                for gameobject_id in loot_gameobject_ids.iter() {
                    positions.append(&mut "SELECT position_x, position_y, position_z FROM gameobject WHERE id = :id".with(params! { "id" => gameobject_id })
                        .map(pool, |(x, y, z)| Position { x, y, z }).unwrap());
                }
                return Some(QuestObjective::CollectFromGameobject {
                    item_id,
                    amount,
                    gameobject_ids: loot_gameobject_ids,
                    positions
                });
            } else {
                unreachable!()
            }
        }
    }

    pub fn export(&self, file: &mut File) {
        if let QuestObjective::KillAndLoot { npc_ids, areas, loot_item, amount } = &self {
            let _ = file.write_all(format!("                        new KillAndLootQuestObjective(wowInterface, new List<int> {{ {} }}, {}, {}, new List<List<Vector3>> {{\n",
                                           npc_ids.iter().map(|val| val.to_string()).collect::<Vec<String>>().join(","), amount, loot_item.unwrap_or(0), ).as_bytes());
            for area in areas.iter() {
                let _ = file.write_all(b"                            new()\n");
                let _ = file.write_all(b"                            {\n");
                for position in area.0.iter() {
                    let _ = file.write_all(format!("                                new Vector3({:.2}f, {:.2}f, {:.2}f),\n", position.x, position.y, position.z).as_bytes());
                }
                let _ = file.write_all(b"                            },\n");
            }
            let _ = file.write_all(b"                        }),\n");
        } else {
            unimplemented!()
        }
    }
}