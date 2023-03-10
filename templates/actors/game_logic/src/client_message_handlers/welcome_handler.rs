use shared::*;
use crate::info_::info_;
use crate::messaging_::publish_;
use crate::spawn_::spawn;
use wasmcloud_interface_messaging::{PubMessage};
use std::collections::HashMap;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::sync::{Arc, Mutex};
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_numbergen::random_in_range;
pub async fn _fn (map:Arc<Mutex<App>>,game_id:String,ball_id:BallId,ball_label:BallLabel)-> RpcResult<()>{
  let (x,y) = if ball_id.1{
    let x = random_in_range(0,900).await? as f32;
    let y = -100.0;
    (-x,y)
  }else{
    let x = random_in_range(0,900).await? as f32;
    let y = -100.0;
    (x,y)
  };
    
    let ball_bundle = BallBundle{
      ball_id:ball_id,
      ball_label:ball_label.clone(),
      transform:Transform { translation: [x,y,3.0].into(), ..Default::default() },
      global_transform:GlobalTransform::identity(),
      velocity:Velocity::zero(),
      rigid_body:RigidBody::Dynamic,
      locked_axes:LockedAxes::ROTATION_LOCKED,
      interpolated:TransformInterpolation::default(),
      collider:Collider::ball(50.0)
    };
    {
      let guard = match map.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
          info_(format!("poisoned {:?}",poisoned));
          poisoned.into_inner()
        },
      };
      let mut app = guard;
      info_(format!("welcome {:?}",ball_bundle.clone()));
      spawn(&mut app.world,ball_bundle.clone());
      let server_message = ServerMessage::Welcome{ball_bundle};
      match rmp_serde::to_vec(&server_message){
        Ok(b)=>{
          let p_msg = PubMessage{
            body:b,
            reply_to: None,
            subject: format!("{}.{}",multitenary::UNIQUE,String::from("welcome"))
            };
          publish_(p_msg);
        }
        _=>{}
      }
      let mut ball_bundles =vec![];
      let mut query = app.world.query::<(&BallId,&BallLabel,&Transform, &Velocity)>();
      for (gball_id,ball_label,transform,velocity) in query.iter(&app.world){
        if gball_id.0!=ball_id.0{//don't send yourself
          ball_bundles.push(BallBundle{ball_id:gball_id.clone(),ball_label:ball_label.clone(),
            transform:transform.clone(),global_transform:GlobalTransform::identity(),
            velocity:velocity.clone(),rigid_body:RigidBody::Dynamic,
            locked_axes:LockedAxes::ROTATION_LOCKED,
            interpolated:TransformInterpolation::default(),
            collider:Collider::ball(50.0)});
        }
      }
      let mut query = app.world.query::<(&VolleyBall,&Transform,&Velocity)>();
      let mut volleyball_bundle = VolleyBallBundle{
        volley_id: VolleyBall{
          last_touch:None
        },
        global_transform: GlobalTransform::identity(),
        locked_axes:LockedAxes::ROTATION_LOCKED,
        transform: Transform::from_xyz(0.0,0.0,0.0),
        velocity: Velocity::zero(),
        rigid_body:RigidBody::Dynamic,
        interpolated: TransformInterpolation::default(),
        collider: Collider::ball(50.0)
      };
      for (_,t,v) in query.iter(&app.world){
        volleyball_bundle.transform = t.clone();
        volleyball_bundle.velocity = v.clone();
        info_(format!("volleyball_bundle {:?}",volleyball_bundle.clone()));
      }
      let channel_message_back = ServerMessage::GameState{ball_bundles:ball_bundles,volleyball_bundle:volleyball_bundle};
      match rmp_serde::to_vec(&channel_message_back){
        Ok(b)=>{
          let p_msg = PubMessage{
            body:b,
            reply_to: None,
            subject: format!("{}.game_logic_specify.{}",multitenary::UNIQUE,ball_id.0)
          };
          publish_(p_msg);
        }
        Err(e)=>{
          info_(format!("m iter ....error{}",e));
        }
      }
    }
    Ok(())
}