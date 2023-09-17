use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

pub fn setup(
    mut commands: Commands,
) {

    // Rectangle
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(50.0, 740.0)),
            ..default()
        },
        ..default()
    })
    .insert_bundle(TransformBundle::from_transform(Transform::from_xyz(0.0,-50.0,2.0)));

}