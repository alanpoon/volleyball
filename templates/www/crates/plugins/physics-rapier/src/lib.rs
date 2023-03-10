use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use bevy_rapier2d::prelude::*;
use shared::*;
mod shape;
mod system;
pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .insert_resource(RapierConfiguration {
                timestep_mode: TimestepMode::Variable {
                    max_dt: 1.0 / 60.0,
                    time_scale: 1.0,
                    substeps: 1,
                },
                gravity: Vec2{x:0.0,y:-300.0},
                ..Default::default()
            })
            .add_system(system::add_player_shape)
            .add_system(system::add_volleyball_shape)
            .add_startup_system(shape::setup);
    }
}
