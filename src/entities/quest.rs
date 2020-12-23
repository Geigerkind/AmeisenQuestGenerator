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
        // Quest_id == 0 => Grinder
        if quest_id == 0 {
            let args: Vec<String> = std::env::args().collect();
            let name = args.get(3).expect("Malformed Name").clone();
            let min_level = u8::from_str_radix(&args.get(4).expect("Malformed MinLevel"), 10).expect("Malformed MinLevel");
            let max_level = u8::from_str_radix(&args.get(5).expect("Malformed MaxLevel"), 10).expect("Malformed MaxLevel");
            let npc_ids = args.get(6).expect("Malformed NpcIds").trim().split(",")
                .map(|npc_id| u32::from_str_radix(npc_id, 10).expect("Malformed NpcId")).collect::<Vec<u32>>();
            Quest {
                id: 0,
                min_level,
                name,
                start: QuestHolder::None,
                end: QuestHolder::None,
                objectives: vec![
                    QuestObjective::new_grind_objective(pool, npc_ids, max_level)
                ]
            }
        } else {
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
        if self.id == 0 {
            let _ = file.write_all(format!("    class {} : GrindingBotQuest\n", quest_name).as_bytes());
        } else {
            let _ = file.write_all(format!("    class {} : BotQuest\n", quest_name).as_bytes());
        }
        let _ = file.write_all(b"    {\n");
        let _ = file.write_all(format!("        public {}(WowInterface wowInterface)\n", quest_name).as_bytes());
        if self.id == 0 {
            let _ = file.write_all(format!("            : base(\"{}\",\n", self.name).as_bytes());
        } else {
            let _ = file.write_all(format!("            : base(wowInterface, {}, \"{}\", {}, 1,\n", self.id, self.name, self.min_level).as_bytes());
            self.start.export(&mut file);
            self.end.export(&mut file);
        }
        if self.objectives.is_empty() {
            let _ = file.write_all(b"                null)\n");
        } else {
            let _ = file.write_all(b"                new List<IQuestObjective>()\n");
            let _ = file.write_all(b"                {\n");
            let _ = file.write_all(b"                    new QuestObjectiveChain(new List<IQuestObjective>()\n");
            let _ = file.write_all(b"                    {\n");
            for objective in self.objectives.iter() {
                objective.export(&mut file);
            }
            let _ = file.write_all(b"                    })\n");
            let _ = file.write_all(b"                })\n");
        }
        let _ = file.write_all(b"        {}\n");
        let _ = file.write_all(b"    }\n");
        let _ = file.write_all(b"}\n");
    }
}