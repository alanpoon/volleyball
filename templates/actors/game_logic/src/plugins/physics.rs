use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::math::Vect;
use bevy_rapier2d::plugin::TimestepMode;
pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
  fn build(&self, app: &mut App) {
      app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
          .insert_resource(RapierConfiguration {
            //scale:1.0,
            gravity: Vec2{x:0.0,y:-300.0},
            timestep_mode: TimestepMode::Interpolated{
              dt:1.0/60.0,
              time_scale:1.0,
              substeps:1,
            },
            ..Default::default()
          })
          ;
  }
}
use shared::*;