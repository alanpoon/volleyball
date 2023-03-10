use bevy::math::Vec2;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use nats_lite::nats;
use protocol::Command;
use shared::*;
pub fn target_velocity(
    ball_id: BallId,
    target_velocity_x: f32,
    target_velocity_y: f32,
    vel: &mut Velocity,
) -> Vec<Command> {
    let t_v = Vec2::new(target_velocity_x, target_velocity_y);
    let tv = ClientMessage::TargetVelocity {
        game_id: String::from("hello"),
        ball_id: ball_id,
        target_velocity: t_v.clone(),
    };

    let tv_ = rmp_serde::to_vec(&tv).unwrap();
    let n1 = nats::proto::ClientOp::Pub {
        subject: String::from("client_handler.hello"),
        reply_to: None,
        payload: tv_,
    };
    let tv = ServerMessage::TargetVelocity {
        ball_id: ball_id,
        target_velocity: t_v.clone(),
    };
    let tv_ = rmp_serde::to_vec(&tv).unwrap();
    let n2 = nats::proto::ClientOp::Pub {
        subject: format!("game_logic.peer"),
        reply_to: None,
        payload: tv_,
    };
    update::target_velocity::velocity(vel,t_v);
    info!("vel,{:?} t_v {:?}",vel,t_v.clone());
 
    vec![
        Command::Nats(String::from("default"), n1),
        Command::Nats(String::from("default"), n2),
    ]
}