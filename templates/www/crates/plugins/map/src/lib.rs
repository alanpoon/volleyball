use bevy::prelude::*;
use bevy::render::texture::ImageSettings;
use chrono::prelude::*;
mod helpers;
mod layer;
use shared::*;
pub struct MapPlugin;
impl Plugin for MapPlugin {
  fn build(&self, app: &mut bevy::app::App) {
      app
         .insert_resource(ImageSettings::default_nearest())
         .add_startup_system(startup)
         .add_startup_system(layer::start_up_layer)
         .add_system(helpers::camera::movement)
         ;
  }
}
#[derive(Component,Clone,Debug)]
pub struct ScoreText();
fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_handle = asset_server
    .load("fonts/FiraSans-Bold.ttf");
    commands.spawn_bundle(TextBundle {
    style: Style {
        align_self: AlignSelf::FlexEnd,
        //position_type: PositionType::Absolute,
        ..Default::default()
    },
    text: Text {
        sections: vec![TextSection {
            value: "Physics time0.1234567890".to_string(),
            style: TextStyle {
                font: font_handle.clone(),
                font_size: 25.0,
                color: Color::BLACK,
                ..Default::default()
            },
            ..Default::default()
        }],
        ..Default::default()
    },
    ..Default::default()
  }).insert(ScoreText());
}