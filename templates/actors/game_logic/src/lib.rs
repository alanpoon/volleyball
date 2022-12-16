extern crate wasmcloud_interface_messaging as messaging;
mod host_call;
mod info_;
mod systems;
mod thread;
mod messaging_;
mod client_message_handlers;
mod plugins;
mod startup;
mod spawn_;
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_messaging::{MessageSubscriber,SubMessage};
use wasmcloud_interface_thread::{StartThreadRequest, StartThreadResponse,Thread,ThreadReceiver,ThreadSender};
use messaging::*;
use crate::thread::thread_handle_request;
use lazy_static::lazy_static;
use bevy::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use serde::{Serialize,Deserialize};
use std::boxed::Box;
use shared::*;
use crate::plugins::physics::PhysicsPlugin;
lazy_static! {
  static ref APP: Arc<Mutex<App>> = Arc::new(Mutex::new(App::new()));
}

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor,Thread,MessageSubscriber)]
struct GameLogicActor {}
#[async_trait]
impl Thread for GameLogicActor{
  async fn start_thread(&self, ctx: &Context, start_thread_request: &StartThreadRequest) -> RpcResult<StartThreadResponse> {
    info!("start_thread----");
    {
      let map = APP.clone();
      let mut m = map.lock().unwrap();
      m.init_resource::<Time>()
      .add_plugin(TransformPlugin::default())
      .add_plugin(PhysicsPlugin)
      .add_plugin(SharedPlugin)
      ;
      
    }
    let provider = ThreadSender::new();
    if let Err(e) = provider
        .start_thread(
            ctx,
            start_thread_request,
        )
        .await
    {
        error!("sending reply: {}",e.to_string());
    }
    info!("end_thread----");
    Ok(StartThreadResponse{})
  }
  async fn tick(&self, _ctx: &Context, start_thread_request: &u64) -> RpcResult<u32> {
    let map = APP.clone();
    thread_handle_request(map,start_thread_request).await
  }
}
#[async_trait]
impl MessageSubscriber for GameLogicActor{
  async fn handle_message(&self, _ctx: &Context, req: &SubMessage) -> RpcResult<()> {
    if req.subject.contains("client_handler"){
      let client_message: Result<ClientMessage,_> = rmp_serde::from_slice(&req.body);
      match client_message{
        Ok(ClientMessage::TargetVelocity{game_id,ball_id,target_velocity})=>{
          let map = APP.clone();
          client_message_handlers::target_velocity_handler::_fn(map,game_id,ball_id,target_velocity);  
        }
        Ok(ClientMessage::Welcome{game_id,ball_id,ball_label})=>{
          let map = APP.clone();
          client_message_handlers::welcome_handler::_fn(map,game_id,ball_id,ball_label).await;
        }
        
        Err(e)=>{
          info!("client_message err {:?}",e);
        }
      }
    }
    Ok(())
  }
}