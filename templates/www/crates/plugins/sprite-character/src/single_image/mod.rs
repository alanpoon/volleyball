use bevy::prelude::*;
use std::collections::HashMap;
pub fn startup(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>,
  mut texture_hashmap:ResMut<HashMap<String,Handle<TextureAtlas>>>) {
  let background_texture = asset_server.load("2d/background.png");
  commands.spawn_bundle(SpriteBundle {
    texture: background_texture,
    //transform: Transform::from_xyz(-960.0, 720.0, 0.0),
    transform: Transform::from_xyz(0.0, 280.0, 0.0),
    ..Default::default()
  });
  //mut_rect(&mut texture_atlases,&mut texture_hashmap,background_texture,String::from("background"),1920.0,1440.0);  
}
pub fn mut_rect(texture_atlases: &mut ResMut<Assets<TextureAtlas>>,texture_hashmap: &mut ResMut<HashMap<String,Handle<TextureAtlas>>>,
  texture:Handle<Image>,key:String,width:f32,length:f32){
  let texture_atlas = TextureAtlas{
    texture: texture,
    size: Vec2::new(width,length),
    textures:vec![bevy::sprite::Rect{min:
      Vec2::new(0.0,0.0),
      max:Vec2::new(width,length)}],
    texture_handles:None,
  };
  let handle = texture_atlases.add(texture_atlas.clone());
  texture_hashmap.insert(key,handle);
}