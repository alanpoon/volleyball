use core::DeskSystem;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use shared::{BallId,systems};
pub struct PhysicsPlugin;
const LINEAR_DAMPING: f32 = 8.0;
use crate::nalgebra::Vector2;
use std::f32::consts::PI;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

const RAPIER_SCALE: f32 = 20.0;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData,>::default())    
            .insert_resource(RapierConfiguration {
                scale: 1.0,
                gravity: Vector2{x:0.0,y:-300.0},
                ..Default::default()
            })
            .add_plugin(RapierDebugRenderPlugin::default());
           
    }
}
