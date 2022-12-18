use bevy::prelude::*;
use shared::*;
use crate::new_ball;
pub struct RadioButtonPlugin;
#[derive(Component)]
pub struct RadioButtonGroup {
    pub entities: Vec<Entity>,
}
#[derive(Component)]
pub struct RadioButton {
    pub selected: bool,
    pub value: String
}
#[derive(Component)]
pub struct RadioButtonGroupRelation(pub Entity);
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);
impl Plugin for RadioButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(radio_button_system.label("radio_button_system"));
        app.add_system(
            radio_button_group_system
                .label("radio_button_group_system")
                .after("radio_button_system"),
        );
    }
}
pub fn radio_button_group_setup(
    mut cmd:Commands,asset_server: Res<AssetServer>
){
    let mut tool_button_ids = vec![];
    let team_a = cmd
    .spawn_bundle(ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(150.0), Val::Px(65.0)),
            // center button
            margin: UiRect::all(Val::Auto),
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,
            ..default()
        },
        transform:Transform::from_xyz(-940.0,720.0,3.0),
        color: NORMAL_BUTTON.into(),
        ..default()
    })
    .with_children(|parent| {
        parent.spawn_bundle(TextBundle::from_section(
            "Team A",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 40.0,
                color: Color::rgb(0.9, 0.9, 0.9),
            },
        ));
    }).insert(RadioButton {
        selected: false,
        value: String::from("Team A")
    }).id();
    tool_button_ids.push(team_a);
    let team_b = cmd
    .spawn_bundle(ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(150.0), Val::Px(65.0)),
            // center button
            margin: UiRect::all(Val::Auto),
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,
            ..default()
        },
        transform:Transform::from_xyz(-940.0,720.0,3.0),
        color: NORMAL_BUTTON.into(),
        ..default()
    })
    .with_children(|parent| {
        parent.spawn_bundle(TextBundle::from_section(
            "Team B",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 40.0,
                color: Color::rgb(0.9, 0.9, 0.9),
            },
        ));
    }).insert(RadioButton {
        selected: false,
        value:String::from("Team B")
    }).id();
    tool_button_ids.push(team_b);
    let tool_group_id = cmd
    .spawn().insert(RadioButtonGroup {
        entities: tool_button_ids.clone(),
    })
    .id();
    for id in tool_button_ids.iter() {
        cmd
        .entity(*id)
        .insert(RadioButtonGroupRelation(tool_group_id));
    }
}
pub fn radio_button_group_system(
    mut q: ParamSet<(
        Query<(Entity, &RadioButton, &RadioButtonGroupRelation,&mut UiColor), Changed<RadioButton>>,
        Query<(&mut RadioButton,&mut UiColor)>,
    )>,
    q_radio_group: Query<&RadioButtonGroup>,
    mut commands: ResMut<protocol::Commands>,
    mut local_user_info: ResMut<LocalUserInfo>,
    mut to_despawn: ResMut<EntityToRemove>,
) {
    //let mut unselect = vec![];
    for (entity, radio, group_rel,mut color) in q.p0().iter_mut() {
        info!("there is relationship");
        if let Ok(radio_group) = q_radio_group.get(group_rel.0) {
            if radio.selected {
                *color = PRESSED_BUTTON.into();
                if &radio.value=="Team A"{
                    new_ball::new_ball(&mut commands,&mut local_user_info,true);
                }else if &radio.value=="Team B"{
                    new_ball::new_ball(&mut commands,&mut local_user_info,false);
                }
                
                for other_entity in radio_group.entities.iter() {
                    to_despawn.entities.insert(*other_entity);
                    // if *other_entity != entity {
                    //     unselect.push(*other_entity);
                    // }
                }
            }
        }
    }

    // for entity in unselect.iter() {
    //     info!("there is unselect");
        
    //     // if let Ok((mut other_radio,mut color)) = q.p1().get_mut(*entity) {
    //     //     other_radio.selected = false;
    //     //     //*color = NORMAL_BUTTON.into();
            
    //     // }
    // }
}

pub fn radio_button_system(
    mut interaction_query: Query<
        (&mut RadioButton, &Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>, With<RadioButton>),
    >,
) {
    for (mut radio, interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                info!("clicked...{:?}",color);
                *color = PRESSED_BUTTON.into();
                
                radio.selected = true;
            }
            Interaction::Hovered => *color = HOVERED_BUTTON.into(),
            Interaction::None => {},
        }
    }
}