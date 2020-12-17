use mysql::Pool;
use mysql::prelude::{BinQuery, WithParams};

use crate::entities::{QuestHolder, QuestObjective};

#[derive(Debug)]
pub struct Quest {
    pub id: u32,
    pub min_level: u8,
    pub name: String,
    pub start: QuestHolder,
    pub end: QuestHolder,
    pub objectives: Vec<QuestObjective>,
}

impl Quest {
    pub fn new(pool: &Pool, quest_id: u32) -> Self {
        let quest_info: Option<(u8, String, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32)> =
            "SELECT MinLevel, LogTitle, RequiredNpcOrGo1, RequiredNpcOrGo2, RequiredNpcOrGo3, RequiredNpcOrGo4, \
               RequiredItemId1, RequiredItemId2, RequiredItemId3, RequiredItemId4, RequiredItemId5, RequiredItemId6 FROM quest_template WHERE ID = :id"
                .with(params! { "id" => quest_id }).first(pool).unwrap();

        if let Some((min_level, name, npc_or_gameobject1, npc_or_gameobject2, npc_or_gameobject3, npc_or_gameobject4,
                        item_id1, item_id2, item_id3, item_id4, item_id5, item_id6)) = quest_info {
            let start = QuestHolder::new(pool, quest_id, true);
            let end = QuestHolder::new(pool, quest_id, false);
            let npc_or_gamobject_ids = vec![npc_or_gameobject1, npc_or_gameobject2, npc_or_gameobject3, npc_or_gameobject4];
            let item_ids = vec![item_id1, item_id2, item_id3, item_id4, item_id5, item_id6];

            let mut objectives = Vec::new();
            for (index, npc_or_gameobject) in npc_or_gamobject_ids.into_iter().enumerate() {
                if npc_or_gameobject == 0 {
                    continue;
                }
                if let Some(kill_objective) = QuestObjective::try_new_kill_objective(pool, quest_id, index + 1, npc_or_gameobject) {
                    objectives.push(kill_objective);
                } else {
                    unimplemented!()
                    // TODO: Gameobject else panic
                }
            }

            for (index, item_id) in item_ids.into_iter().enumerate() {
                if item_id == 0 {
                    continue;
                }
                if let Some(collect_objective) = QuestObjective::try_new_collect_objective(pool, quest_id, index + 1, item_id) {
                    objectives.push(collect_objective);
                } else {
                    unimplemented!()
                    // Hmm?
                }
            }

            return Quest {
                id: quest_id,
                min_level,
                name,
                start,
                end,
                objectives,
            };
        } else {
            panic!("No quest found with this ID!");
        }
    }
}