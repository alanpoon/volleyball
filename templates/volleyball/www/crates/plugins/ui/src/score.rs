use bevy::prelude::*;
use bevy::sprite::Rect;
use crate::*;
use shared::*;
pub fn score_setup(mut commands:Commands,asset_server: Res<AssetServer>){
    commands
    .spawn_bundle(TextBundle {
        style: Style {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(5.0),
                left: Val::Px(15.0),
                ..Default::default()
            },
            ..Default::default()
        },
        // Use the `Text::with_section` constructor
        text: Text::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "Team A:\n",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 100.0,
                color: Color::WHITE,
            }
        ).with_alignment(TextAlignment::CENTER),
        ..Default::default()
    }).insert(TeamAScore);
    commands
    .spawn_bundle(TextBundle {
        style: Style {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(5.0),
                right: Val::Px(15.0),
                ..Default::default()
            },
            ..Default::default()
        },
        // Use the `Text::with_section` constructor
        text: Text::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "Team B:\n",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 100.0,
                color: Color::WHITE,
            }
        ).with_alignment(TextAlignment::CENTER),
        ..Default::default()
    }).insert(TeamBScore);
}
pub fn score_update(mut query_a:Query<&mut Text, (With<TeamAScore>,Without<TeamBScore>)>,mut query_b:Query<&mut Text,(With<TeamBScore>,Without<TeamAScore>)>,score:Res<scoreboard::ScoreBoard>){
    for mut text in query_a.iter_mut() {
        let score = score.a;
        text.sections[0].value = format!("Team A\n{:?}",score);
    }
    for mut text in query_b.iter_mut() {
        let score = score.b;
        text.sections[0].value = format!("Team B\n{:?}",score);
    }
}
pub fn spawn_score_animation(mut commands:Commands,query:Query<(Entity,&ScoreAnimation),Added<ScoreAnimation>>,asset_server:Res<AssetServer>){
    for (e,s) in query.iter(){
        let pos = if s.0{
            UiRect {
                top: Val::Px(5.0),
                left: Val::Px(15.0),
                ..Default::default()
            }
        }else{
            UiRect {
                top: Val::Px(5.0),
                right: Val::Px(15.0),
                ..Default::default()
            }
        };
        commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: pos,
                ..Default::default()
            },
            // Use the `Text::with_section` constructor
            text: Text::from_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                "+1",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 30.0,
                    color: Color::YELLOW,
                }
            ).with_alignment(TextAlignment::CENTER),
            ..Default::default()
        }).insert(PlusOne(e,Timer::from_seconds(2.0,false)));
    }
}
pub fn score_animate(mut query:Query<(Entity,&mut PlusOne)>,mut to_despawn:ResMut<EntityToRemove>,time:Res<Time>){
    for (entity, mut plus_one) in query.iter_mut() {
        plus_one.1.tick(time.delta());
        if plus_one.1.just_finished() {
          to_despawn.entities.insert(plus_one.0);
          to_despawn.entities.insert(entity);
        }
       
      }
}
