use bevy::prelude::*;
use shared::*;
mod new_ball;
mod radio_button;
mod score;

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
        ).add_system(score::score_update)
        .add_system(score::spawn_score_animation)
        .add_system(score::score_animate)
        .add_startup_system(score::score_setup);
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
#[derive(Component,Clone,Debug)]
pub struct TeamAScore;
#[derive(Component,Clone,Debug)]
pub struct TeamBScore;
#[derive(Component,Clone,Debug)]
pub struct PlusOne(Entity,pub Timer);//ScoreAnimation's entity