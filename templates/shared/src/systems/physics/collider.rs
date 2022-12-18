use bevy_rapier2d::prelude::*;
use bevy::prelude::*;
use bevy_rapier2d::math::Vect;
use crate::*;

pub fn collider_setup(mut cmd:Commands){
    //ground
    cmd.spawn_bundle(TransformBundle::from(Transform::from_xyz(
        -940.0,
        -300.0,
        3.0,
    )))
    .insert(Collider::cuboid(2000.0, 10.0)).insert(OutOfBound);
    //net
    cmd.spawn_bundle(TransformBundle::from(Transform::from_xyz(
        0.0,-50.0,2.0
    )))
   .insert(Collider::cuboid(45.0, 380.0)).insert(OutOfBound);
   //ceiling
    cmd.spawn_bundle(TransformBundle::from(Transform::from_xyz(
    -940.0,
    1000.0,
    2.0,
    )))
    .insert(Collider::cuboid(2000.0, 10.0));
    //left wall
    cmd.spawn_bundle(TransformBundle::from(Transform::from_xyz(-940.0,-720.0,2.0)))
    .insert(Collider::cuboid(50.0, 2040.0)).insert(OutOfBound);
    //right wall
    cmd.spawn_bundle(TransformBundle::from(Transform::from_xyz(940.0,-720.0,2.0)))
    .insert(Collider::cuboid(50.0, 2040.0)).insert(OutOfBound);
}