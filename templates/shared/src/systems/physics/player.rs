use bevy_rapier2d::prelude::*;
use bevy::prelude::*;
use bevy_rapier2d::math::Vect;
use crate::*;
use bevy::utils::Duration;
pub fn update_state_velocity_physics(mut query: Query<(&BallId,&mut Transform,&mut Velocity)>) {
    for (ball_id,mut transform,mut v) in query.iter_mut() {
      let pos_x = transform.translation.x;
      let pos_y = transform.translation.y;
      let mut x=0.0;
      let mut y=0.0;
      if ball_id.1{ //true is left image:width: 1920, height:1444
        if (pos_x<=-940.0 && v.linvel.x >0.0) || (pos_x>=0.0 && v.linvel.x <0.0) || (pos_x>=-940.0 && pos_x <= 0.0){
          x = v.linvel.x;
        }
        // if (pos_y<=20.0 && v.linvel.y >0.0) || (pos_y>=3820.0 && v.linvel.y <0.0) || (pos_y>=20.0 && pos_y <= 3820.0){
        //   y = v.linvel.y;
        // }
        if pos_x>=0.0{
          transform.translation.x = 0.0;
        }
        if pos_x<=-940.0{
          transform.translation.x = -940.0;
        }
      }else{
        if (pos_x<=0.0 && v.linvel.x >0.0) || (pos_x>=940.0 && v.linvel.x <0.0) || (pos_x>=0.0 && pos_x <= 940.0){
          x = v.linvel.x;
        }
        if pos_x>=940.0{
          transform.translation.x = 940.0;
        }
        if pos_x<=0.0{
          transform.translation.x = 0.0;
        }
      }
      
      //let move_delta = Vect::new(x, y);
      v.linvel.x = x;
    }
  }

  pub fn remove_move_by_timer(mut query: Query<(&BallId,&mut Velocity,&mut MoveTimer)>,time : Res<Time>){
    for (ball_id,mut v,mut mv_timer) in query.iter_mut() {
      if !mv_timer.0.finished(){
        let delta = time.delta_seconds();
        // if v.linvel.y>0.0{
          
        // }
        v.linvel.y -= 300.0 * delta;
      }
      // if mv_timer.0.tick(Duration::from_millis((time.delta_seconds() as f32 * 1000.0) as u64)).just_finished() {
      //   v.linvel.y = -;
      // }
    }
  }