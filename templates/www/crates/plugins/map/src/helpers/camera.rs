use bevy::{ input::Input, math::Vec3, prelude::*, render::camera::Camera};
use bevy::time::Time;
// A simple camera system for moving and zooming the camera.
pub fn movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut OrthographicProjection,&mut Camera)>,
    mut text_query: Query<(&mut Text,&mut Style,&mut GlobalTransform)>, 
) {
    for (mut transform, mut ortho, _c) in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::A) {
            direction -= Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::S) {
            direction -= Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::Z) {
            ortho.scale += 0.1;
        }

        if keyboard_input.pressed(KeyCode::X) {
            ortho.scale -= 0.1;
        }

        if ortho.scale < 0.2 {
            ortho.scale = 0.2;
        }

        let z = transform.translation.z;
        transform.translation.z = z;
        transform.translation+= direction;
        for (mut text,mut _s,mut _g)  in text_query.iter_mut() {
            text.sections[0].value = format!(r#"translation {:?} scale {:?}"#,transform.translation,ortho.scale);
        }
    }
}