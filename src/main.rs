extern crate chull;
#[macro_use]
extern crate mysql;
extern crate dbscan;
extern crate plotters;

use plotters::prelude::*;

use mysql::Pool;
use mysql::prelude::*;
use crate::entities::{Quest, QuestObjective};
use crate::value_objects::Position;

pub mod entities;
pub mod value_objects;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = Pool::new("mysql://root:vagrant@localhost:3306/world_wotlk").unwrap();
    let quest = Quest::new(&pool, 4402);

    println!("{:?}", quest);

/*
    // Plotting for debugging!
    let orig_points = "SELECT position_x, position_y, position_z FROM creature WHERE id = :id".with(params! { "id" => 3124 })
        .map(&pool, |(x, y, z)| Position { x, y, z }).unwrap().into_iter().map(|pos| [pos.x + 800.0, pos.y + 4600.0]).collect::<Vec<[f64; 2]>>();


    // Plotting
    let root =
        BitMapBackend::new("test.png", (1024, 768)).into_drawing_area();

    root.fill(&WHITE)?;

    let areas = root.split_by_breakpoints([944], [80]);

    let mut scatter_ctx = ChartBuilder::on(&areas[2])
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0f64..1000f64, 0f64..600f64)?;
    scatter_ctx
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .draw()?;
    scatter_ctx.draw_series(
        orig_points
            .iter()
            .map(|[x,y]| Circle::new((*x, *y), 1, GREEN.filled())))?;

    let colors = vec![BLACK, BLUE, CYAN, MAGENTA, RED, YELLOW, RGBColor(50, 50, 50), RGBColor(100, 50, 50), RGBColor(50, 50, 150), RGBColor(50, 150, 50)];
    if let QuestObjective::KillAndLoot { areas, .. } = &quest.objectives[0] {
        for (idx, area) in areas.iter().enumerate() {
            let points = area.0.iter().map(|pos| [pos.x + 800.0, pos.y + 4600.0]).collect::<Vec<[f64; 2]>>();
            scatter_ctx.draw_series(
                points
                    .iter()
                    .map(|[x,y]| Circle::new((*x, *y), 2, colors[idx].filled())))?;
        }
    } else {
        unreachable!()
    };

 */
    Ok(())
}
