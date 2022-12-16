use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use shared::*;
use protocol::Command;
use bevy::prelude::*;
use nats_lite::nats;
use rand::{thread_rng, Rng};
pub fn new_ball(mut cmd:Commands,mut commands: ResMut<protocol::Commands>,mut local_user_info: ResMut<LocalUserInfo>){
    info!("sending welcome");
    let n = nats::proto::ClientOp::Sub{
        subject:String::from("welcome"),
        queue_group:None,
        sid:16,
    };
    commands.push(Command::Nats(String::from("default"),n));
    let mut rng = thread_rng();
    let n: u32 = rng.gen_range(100000..999999);
    let tv = ClientMessage::Welcome{
        game_id:String::from("hello"),
        ball_id:BallId(n),
        ball_label:BallLabel(n.to_string(),String::from("")),
    };
    let tv_= rmp_serde::to_vec(&tv).unwrap();
    let msg = nats::proto::ClientOp::Pub{
        subject: String::from("client_handler.hello"),
        reply_to: None,
        payload: tv_,
    };
    commands.push(protocol::Command::Nats(String::from("default"),msg));
    let msg = nats::proto::ClientOp::Sub{
        subject:format!("game_logic"),
        queue_group:None,
        sid:17,
      };
    commands.push(protocol::Command::Nats(String::from("default"),msg));
    let msg = nats::proto::ClientOp::Sub{
        subject: format!("game_logic_specify.{}",n),
        queue_group:None,
        sid:21,
    };
    commands.push(protocol::Command::Nats(String::from("default"),msg));
    let msg = nats::proto::ClientOp::Sub{
        subject:format!("game_logic.peer"),
        queue_group:None,
        sid:24,
      };
    commands.push(protocol::Command::Nats(String::from("default"),msg));
    
    // cmd.spawn_bundle(BallBundle{
    //     ball_id:BallId(n),
    //     ball_label:BallLabel(n.to_string(),String::from("")),
    //     transform: Transform::from_xyz(0.0,0.0,3.0).with_scale(Vec3::splat(0.2)),
    //     global_transform:GlobalTransform::identity(),
    //     velocity:Velocity { linvel: [0.0,0.0].into(), ..Default::default() },
    //     rigid_body:RigidBody::Dynamic,
    //     locked_axes:LockedAxes::ROTATION_LOCKED,
    //     interpolated: TransformInterpolation::default()
    // });
    *local_user_info = LocalUserInfo(UserInfo{
        ball_id:BallId(n)
    });
}