use std::collections::HashMap;

use chull::ConvexHullWrapper;
use dbscan::{Classification, cluster};
use mysql::Pool;
use mysql::prelude::{BinQuery, WithParams};

use crate::value_objects::Position;

#[derive(Debug)]
pub struct Area(pub Vec<Position>);

impl Area {
    pub fn new(pool: &Pool, npc_ids: &Vec<u32>) -> Vec<Self> {
        let mut positions = Vec::new();
        for npc_id in npc_ids {
            positions.append(&mut "SELECT position_x, position_y, position_z FROM creature WHERE id = :id".with(params! { "id" => npc_id })
                .map(pool, |(x, y, z)| Position { x, y, z }).unwrap());
        }
        Self::from_positions(positions)
    }

    pub fn from_positions(positions: Vec<Position>) -> Vec<Self> {
        let args: Vec<String> = std::env::args().collect();
        let epsilon = args.get(2).expect("Second argument should be a the clustering expansion radius, e.g. 50.0!")
            .parse::<f64>().expect("Second argument should be a the clustering expansion radius, e.g. 50.0!");

        let clustered = cluster(epsilon, 3, &positions.iter().map(|pos| vec![pos.x, pos.y]).collect::<Vec<Vec<f64>>>());
        let mut max_cluster_id = clustered.iter().map(|classification| match classification {
            Classification::Core(id) | Classification::Edge(id) => *id,
            _ => 0,
        }).max().unwrap();

        let mut cluster = HashMap::new();
        for (idx, classification) in clustered.iter().enumerate() {
            let cluster_id = match classification {
                Classification::Core(id) | Classification::Edge(id) => *id,
                Classification::Noise => {
                    max_cluster_id += 1;
                    max_cluster_id
                }
            };

            let area = cluster.entry(cluster_id).or_insert_with(Vec::new);
            area.push(positions[idx].clone());
        }

        let mut areas = Vec::new();
        for (_, cluster) in cluster {
            if cluster.len() <= 2 {
                for pos in cluster {
                    areas.push(Area(vec![pos]));
                }
            } else {
                let hull = ConvexHullWrapper::try_new(&cluster.iter().map(|pos| vec![pos.x, pos.y]).collect::<Vec<Vec<f64>>>(), None).expect("This must be able to form a hull");
                let (v, _) = hull.vertices_indices();
                areas.push(Area(cluster.into_iter().filter(|pos| v.iter().find(|ipos| ipos[0] == pos.x && ipos[1] == pos.y).is_some()).collect()));
            }
        }
        areas // TODO: Order area's positions?
    }

    pub fn trim_overlapping(areas: Vec<Area>) -> Vec<Self> {
        Area::from_positions(areas.into_iter().fold(Vec::new(), |mut acc, mut area| {
            acc.append(&mut area.0);
            acc
        }))
    }
}