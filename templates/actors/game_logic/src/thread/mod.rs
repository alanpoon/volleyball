use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use wasmcloud_interface_thread::{StartThreadRequest, StartThreadResponse};
use wasmbus_rpc::actor::prelude::*;
use crate::info_::info_;
use crate::startup;
use bevy::prelude::*;
pub async fn thread_handle_request(map:Arc<Mutex<App>>,start_thread_request: &u64)->RpcResult<u32>{
  {
    let guard = match map.try_lock() {
      Ok(guard) => Ok(guard),
      Err(poisoned) => {
        Err(String::from(""))
      },
    };
    if let Err(_)= guard{
      return Ok(0);
    } 
    let mut app = guard.unwrap();
    if let Some(mut t) = app.world.get_resource_mut::<Time>(){
      t.update_with_timestamp(start_thread_request.clone())
    }else{
      app.world.insert_resource(Time::default());
    }
      app.update();
  }
  Ok(0)
}