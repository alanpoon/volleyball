use shared::*;
use crate::info_::info_;
use crate::messaging_::publish_;
use wasmcloud_interface_messaging::{PubMessage};
use std::sync::{Arc, Mutex};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
pub fn _fn (map:Arc<Mutex<App>>,_game_id:String,ball_id:BallId,target_velocity:Vec2){
  let  guard = match map.lock() {
    Ok(guard) => guard,
    Err(poisoned) => {
      poisoned.into_inner()
    },
  };
  let mut app = guard;
  let mut velocity_query= app.world.query::<(Entity,&BallId,&Transform,&mut Velocity)>();
  let mut spawn_timer = None;
  for (e,gball_id,transform,mut velocity) in velocity_query.iter_mut(&mut app.world){
    if gball_id.0 ==ball_id.0{
      update::target_velocity::velocity(&mut velocity, target_velocity.clone());
      spawn_timer = Some(e);
    }
  }
  if let Some(e) = spawn_timer{
    app.world.entity_mut(e)
    .insert(MoveTimer(Timer::from_seconds(1.0,false)));
  }
  
}