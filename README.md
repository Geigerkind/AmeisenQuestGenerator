# AmeisenBotX Quest Generator
This is a utility tool for the [AmeisenBotX](https://github.com/Jnnshschl/AmeisenBotX). It can generate simple quest:
* KillAndLoot, e. g. kill quest or quest where you have to collect loot
* CollectFromGameobject, e. g. collecting items from game objects 
* StartAndEnd, e. g. if a quest just needs to be handed in somewhere

I am not actively working on this tool. If you want to extend it feel free to create a pull request!

## Installation
1. Install cargo and rust stable.
2. Build the docker database container with `docker build -t aqg_database .`.
3. Run the docker container with `docker run -d -p 33306:3306 aqg_database`.
4. Execute in the terminal with `cargo run <quest_id> <cluster expansion radius>`, e. g. `cargo run 788 50.0`.

## Notes
I added a static of the [TrinityCore WotLK](https://github.com/TrinityCore/TrinityCore/tree/3.3.5) DB here, so its easy to set up and invariant to changes.