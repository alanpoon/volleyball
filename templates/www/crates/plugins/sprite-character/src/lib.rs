use bevy::prelude::*;
use std::collections::HashMap;
mod chicken;
mod sprite_sheet;
mod single_image;
use wasm_bindgen::prelude::*;
pub struct SpriteCharacterPlugin;
#[derive(Component,Debug, PartialEq, Default)]
pub struct H{
  pub hash_map:HashMap<String,usize>
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(SystemLabel)]
pub enum SpriteLabel {
    /// everything that handles input
    SpriteSheet,
}
impl Plugin for SpriteCharacterPlugin {
  fn build(&self, app: &mut bevy::app::App) {
      app
      .init_resource::<HashMap<String,Handle<TextureAtlas>>>()
      .init_resource::<H>()
      .init_resource::<Handle<Font>>()
      .add_startup_system(single_image::startup)
      ;
  }
}
use js_sys::{Array};
#[wasm_bindgen]
extern "C" {
    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = window, js_name = f32_flag)]
    fn f32_flags_array() -> Array;
}
use serde::{Deserialize, Serialize};
#[derive(Serialize,Deserialize)]
pub struct Obj{
  pub key: String,
  pub index: usize,
}
//f32_country_array
fn country_flag_startup(mut commands: Commands, asset_server: Res<AssetServer>,
  mut flag_usize_map:ResMut<H>,
  mut font_handle: ResMut<Handle<Font>>) {
  let texture = asset_server.load("2d/round.png");
  //f32_flags_array()
  for j in f32_flags_array().iter(){
    // if let Ok(j2) = j.into_serde::<Obj>(){
    //   (*flag_usize_map).hash_map.insert(j2.key,j2.index);
    // }
    let j2:Result<Obj,_> = serde_wasm_bindgen::from_value(j);
    if let Ok(j2) = j2{
      (*flag_usize_map).hash_map.insert(j2.key,j2.index);
    }
  }
  commands.insert_resource(sprite_sheet::SpriteInfos {
    _2d_round:(texture,Vec2::new(
      871.0,
      859.0,
    )),
  });
  *font_handle = asset_server
  .load("fonts/FiraSans-Bold.ttf");
}
#[derive(Component,Clone,Debug)]
pub struct AnimationTimer(Timer);