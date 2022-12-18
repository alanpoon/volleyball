use bevy::prelude::*;
use shared::*;
pub mod new_ball;
pub mod radio_button;


pub struct UiPlugin;
impl Plugin for UiPlugin {
  fn build(&self, app: &mut bevy::app::App) {
      app
        .add_startup_system(radio_button::radio_button_group_setup)
        //.add_system(button_system)
        .add_system(radio_button::radio_button_system.label("radio_button_system"))
        .add_system(
        radio_button::radio_button_group_system
            .label("radio_button_group_system")
            .after("radio_button_system"),
        );
  }
}

fn button_system(
    mut cmd:Commands,

    mut interaction_query: Query<
        (Entity,&Interaction, &mut UiColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut commands: ResMut<protocol::Commands>,
    mut local_user_info: ResMut<LocalUserInfo>,
    mut to_despawn: ResMut<EntityToRemove>,
) {
    for (e,interaction, mut color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                if text.sections[0].value=="Team A"{
                    new_ball::new_ball(&mut commands,&mut local_user_info,true);
                    to_despawn.entities.insert(e);

                }else if text.sections[0].value=="Team B"{
                    new_ball::new_ball(&mut commands,&mut local_user_info,false);
                    to_despawn.entities.insert(e);
                }
                
            }
           _=>{}
        }
    }
}
// fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    
// }