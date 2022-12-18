use bevy::prelude::*;
use std::collections::HashSet;
use serde::{Deserialize, Serialize};
mod bundle;
mod systems;
mod plugin;
mod scoreboard;
pub use bundle::*;
pub use systems::*;
pub mod to_despawn;
pub mod update;
pub use plugin::SharedPlugin;
#[derive(Component,Serialize, Deserialize, Default, Clone, Copy, Debug, PartialEq, Hash, Eq)]
pub struct BallId(pub u32,pub bool);// id, is_left
#[derive(Debug,Clone,Serialize,Deserialize)]
pub enum ServerMessage {
    GameState{ball_bundles:Vec<BallBundle>,volleyball_bundle:VolleyBallBundle},
    TargetVelocity{ball_id:BallId,target_velocity:Vec2},
    Welcome{ball_bundle:BallBundle},
    Scores{scoreboard:scoreboard::ScoreBoard},
}
#[derive(Component,Serialize, Deserialize, Default, Clone,Debug, PartialEq, Hash, Eq)]
pub struct BallLabel(pub String,pub String); //Label, Flag
#[derive(Serialize, Deserialize, Clone)]
pub enum ClientMessage {
    TargetVelocity{game_id:String,ball_id:BallId,target_velocity:Vec2},
    Welcome{game_id:String,ball_id:BallId,ball_label:BallLabel},
    Jump{ball_id:BallId,velocity:Vec2}
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone,Default)]
pub struct UserInfo{
  pub ball_id:BallId,
}
#[derive(Component,Default,Debug)]
pub struct LocalUserInfo(pub UserInfo);

#[derive(Default,Debug,Clone)]
pub struct EntityToRemove{
  pub entities: HashSet<Entity>
}
#[derive(Component,Serialize, Deserialize, Default, Clone, Debug)]
pub struct Dash(pub bool,pub Vec2,pub Vec2); //on/off, new_speed, old speed
#[derive(Component,Clone,Debug)]
pub struct MoveTimer(pub Timer);
#[derive(Component,Clone,Debug)]
pub struct CoolDownTimer(pub Timer,pub String);
#[derive(Component,Clone,Debug,Serialize, Deserialize)]
pub struct VolleyBall{
  pub last_touch:Option<LastTouch>
}
#[derive(Clone,Debug,Serialize, Deserialize)]
pub struct LastTouch(pub u32,pub bool); //ball_id, is_left
#[derive(Component,Clone,Debug,Serialize, Deserialize)]
pub struct OutOfBound;