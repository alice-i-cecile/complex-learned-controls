use crate::building_blocks::{Point, PointBundle, SpringBundle};
use bevy::app::StartupStage;
use bevy::app::{AppBuilder, Plugin};
use bevy::ecs::entity::Entity;
use bevy::ecs::query::With;
use bevy::ecs::system::{Commands, IntoSystem, Query, Res};
use itertools::Itertools;

pub struct InitializationPlugin;

impl Plugin for InitializationPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(InitialPoints { n: 10 })
            .insert_resource(ArenaSize { x: 400.0, y: 600.0 })
            .add_startup_system(initialize_points.system())
            .add_startup_system(initialize_arena.system())
            // Must run in another stage to allow points to spawn first
            .add_startup_system_to_stage(StartupStage::PostStartup, initialize_springs.system());
    }
}

struct InitialPoints {
    pub n: usize,
}
pub struct ArenaSize {
    pub x: f32,
    pub y: f32,
}

fn initialize_points(mut commands: Commands, config: Res<InitialPoints>) {
    for _ in 0..config.n {
        commands.spawn(PointBundle::new());
    }
}

fn initialize_springs(mut commands: Commands, points: Query<&Entity, With<Point>>) {
    for (point_i, point_j) in points.iter().cartesian_product(points.iter()) {
        commands.spawn(SpringBundle::new((*point_i, *point_j)));
    }
}

fn initialize_arena(mut commands: Commands) {}
