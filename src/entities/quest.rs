use mysql::Pool;
use mysql::prelude::{BinQuery, WithParams};

use crate::entities::{QuestHolder, QuestObjective};
use std::fs::File;
use change_case::pascal_case;
use std::io::Write;

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

    pub fn export(&self) {
        let quest_name = format!("Q{}", pascal_case(&self.name.replace("'s", "").replace(" ", "_")));
        let mut file = File::create(&format!("export/{}.cs", quest_name)).unwrap();
        let _ = file.write_all(b"using AmeisenBotX.Core.Movement.Pathfinding.Objects;\n");
        let _ = file.write_all(b"using AmeisenBotX.Core.Quest.Objects.Objectives;\n");
        let _ = file.write_all(b"using AmeisenBotX.Core.Quest.Objects.Quests;\n");
        let _ = file.write_all(b"using System.Collections.Generic;\n");
        let _ = file.write_all(b"\n");
        let _ = file.write_all(b"namespace AmeisenBotX.Core.Quest.Quests.TODO\n");
        let _ = file.write_all(b"{\n");
        let _ = file.write_all(format!("    class {} : BotQuest\n", quest_name).as_bytes());
        let _ = file.write_all(b"    {\n");
        let _ = file.write_all(format!("        public {}(WowInterface wowInterface)\n", quest_name).as_bytes());
        let _ = file.write_all(format!("            : base(wowInterface, {}, \"{}\", {}, 1,\n", self.id, self.name, self.min_level).as_bytes());
        if let QuestHolder::Npc { id, position } = &self.start {
            let _ = file.write_all(format!("                () => (wowInterface.ObjectManager.GetClosestUnitByNpcId(new List<int> {{ {} }}), new Vector3({:.2}f, {:.2}f, {:.2}f)),\n", id, position.x, position.y, position.z).as_bytes());
        } else {
            unimplemented!()
        }
        if let QuestHolder::Npc { id, position } = &self.end {
            let _ = file.write_all(format!("                () => (wowInterface.ObjectManager.GetClosestUnitByNpcId(new List<int> {{ {} }}), new Vector3({:.2}f, {:.2}f, {:.2}f)),\n", id, position.x, position.y, position.z).as_bytes());
        } else {
            unimplemented!()
        }
        let _ = file.write_all(b"                new List<IQuestObjective>()\n");
        let _ = file.write_all(b"                {\n");
        let _ = file.write_all(b"                    new QuestObjectiveChain(new List<IQuestObjective>()\n");
        let _ = file.write_all(b"                    {\n");
        for objective in self.objectives.iter() {
            if let QuestObjective::KillAndLoot { npc_ids, areas, loot_item, amount } = objective {
                let _ = file.write_all(format!("                        new KillAndLootQuestObjective(wowInterface, new List<int> {{ {} }}, {}, {}, new List<List<Vector3>> {{\n",
                                               npc_ids.iter().map(|val| val.to_string()).collect::<Vec<String>>().join(","), amount, loot_item.unwrap_or(0), ).as_bytes());
                for area in areas.iter() {
                    let _ = file.write_all(b"                            new()\n");
                    let _ = file.write_all(b"                            {\n");
                    for position in area.0.iter() {
                        let _ = file.write_all(format!("                                new Vector3({:.2}f, {:.2}f, {:.2}f),\n", position.x, position.y, position.z).as_bytes());
                    }
                    let _ = file.write_all(b"                            }\n");
                }
                let _ = file.write_all(b"                        }),\n");
            } else {
                unimplemented!()
            }
        }
        let _ = file.write_all(b"                    })\n");
        let _ = file.write_all(b"                })\n");
        let _ = file.write_all(b"        {}\n");
        let _ = file.write_all(b"    }\n");
        let _ = file.write_all(b"}\n");
    }
}