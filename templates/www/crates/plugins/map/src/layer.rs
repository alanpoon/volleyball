use bevy::prelude::*;
pub fn start_up_layer(mut commands: Commands, asset_server: Res<AssetServer>){
  let mut camera = Camera2dBundle::default();
   camera.transform.translation.x = 0.0;
   camera.transform.translation.y = 133.0;
  camera.projection.scale = 2.5;
  commands.spawn_bundle(camera);
}