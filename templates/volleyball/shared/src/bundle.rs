use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::*;
use serde::ser::{Serialize, Serializer};
use serde::de::{Deserialize, Deserializer};

#[derive(Bundle,Clone,Debug)]
pub struct BallBundle {
    pub ball_id: BallId,
    pub ball_label: BallLabel,
    pub global_transform: GlobalTransform,
    pub locked_axes:LockedAxes,
    pub transform: Transform,
    pub velocity: Velocity,
    pub rigid_body:RigidBody,
    pub interpolated: TransformInterpolation,
    pub collider: Collider
}
#[derive(Clone,Debug,Serialize,Deserialize)]
struct BallBundleS {
    pub ball_id: BallId,
    pub ball_label: BallLabel,
    pub transform: Vec2,
    pub velocity: Vec2,
}
impl Serialize for BallBundle{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let bs = BallBundleS{
            ball_id:self.ball_id,
            ball_label:self.ball_label.clone(),
            transform:Vec2::new(self.transform.translation.x,self.transform.translation.y),
            velocity:Vec2::new(self.velocity.linvel.x,self.velocity.linvel.y),
        };
        bs.serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for BallBundle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bs =BallBundleS::deserialize(deserializer)?;
        Ok(BallBundle{
            ball_id:bs.ball_id,
            ball_label:bs.ball_label,
            transform: Transform::from_xyz(bs.transform.x,bs.transform.y,3.0),
            global_transform:GlobalTransform::identity(),
            velocity:Velocity { linvel: [bs.velocity.x,bs.velocity.y].into(), ..Default::default() },
            rigid_body:RigidBody::Dynamic,            
            locked_axes:LockedAxes::ROTATION_LOCKED,
            interpolated: TransformInterpolation::default(),
            collider: Collider::ball(50.0),
        })
    }
}
#[derive(Bundle,Clone,Debug)]
pub struct VolleyBallBundle {
    pub volley_id: VolleyBall,
    pub global_transform: GlobalTransform,
    pub locked_axes:LockedAxes,
    pub transform: Transform,
    pub velocity: Velocity,
    pub rigid_body:RigidBody,
    pub interpolated: TransformInterpolation,
    pub collider: Collider,
    //pub sensor:Sensor
}
#[derive(Clone,Debug,Serialize,Deserialize)]
struct VolleyBallBundleS {
    pub volley_id: VolleyBall,
    pub transform: Vec2,
    pub velocity: Vec2,
}
impl Serialize for VolleyBallBundle{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let bs = VolleyBallBundleS{
            volley_id:self.volley_id.clone(),
            transform:Vec2::new(self.transform.translation.x,self.transform.translation.y),
            velocity:Vec2::new(self.velocity.linvel.x,self.velocity.linvel.y),
        };
        bs.serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for VolleyBallBundle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bs =VolleyBallBundleS::deserialize(deserializer)?;
        Ok(VolleyBallBundle{
            volley_id: bs.volley_id,
            transform: Transform::from_xyz(bs.transform.x,bs.transform.y,3.0),
            global_transform:GlobalTransform::identity(),
            velocity:Velocity { linvel: [bs.velocity.x,bs.velocity.y].into(), ..Default::default() },
            rigid_body:RigidBody::Dynamic,            
            locked_axes:LockedAxes::ROTATION_LOCKED,
            interpolated: TransformInterpolation::default(),
            collider: Collider::ball(50.0),
            //sensor: Sensor
        })
    }
}