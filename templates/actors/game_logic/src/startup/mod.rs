use crate::messaging_::publish_;
use wasmcloud_interface_messaging::PubMessage;
use shared::*;
use std::collections::HashSet;
use crate::info_::info_;
use bevy::utils::Duration;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
pub fn volleyball_setup(mut cmd:Commands){
    cmd.spawn_bundle(VolleyBallBundle{
      volley_id: VolleyBall{
        last_touch:None
      },
      global_transform: GlobalTransform::identity(),
      locked_axes: LockedAxes::ROTATION_LOCKED,
      transform: Transform::from_xyz(-920.0,4.0,2.0),
      velocity: Velocity::zero(),
      rigid_body:RigidBody::Dynamic,
      interpolated: TransformInterpolation::default(),
      collider: Collider::ball(50.0)
    });
}