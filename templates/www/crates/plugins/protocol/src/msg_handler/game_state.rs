use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use shared::*;

pub fn _fn_spawn_or_update_ball_bundles(
    cmd: &mut Commands,
    set: &mut ParamSet<(Query<(Entity, &BallId, &mut Transform, &mut Velocity), With<BallId>>,
    Query<(Entity, &VolleyBall, &mut Transform, &mut Velocity), With<VolleyBall>>,)>,
    ball_bundles: Vec<BallBundle>,
) {
    let len = ball_bundles.len();
    let mut founds = vec![];
    for i in 0..len {
        for (_e, ball_id, mut t, mut v) in set.p0().iter_mut() {
            let ball_bundle = ball_bundles.get(i).unwrap();
            if ball_bundle.ball_id.0 == ball_id.0 {
                *v = ball_bundle.velocity;
                *t = ball_bundle.transform;
                founds.push(i);
                break;
            }
        }
    }
    for (i, ball_bundle) in ball_bundles.iter().enumerate() {
        if !founds.contains(&i) {
            cmd.spawn_bundle(ball_bundle.clone());
        }
    }
}
pub fn _fn_spawn_or_update_volleyball_bundles(
    cmd: &mut Commands,
    set: &mut ParamSet<(Query<(Entity, &BallId, &mut Transform, &mut Velocity), With<BallId>>,
    Query<(Entity, &VolleyBall, &mut Transform, &mut Velocity), With<VolleyBall>>,)>,
    volleyball_bundle: VolleyBallBundle,
) {
    let mut found = false;
    for (_e, volleyball_id, mut t, mut v) in set.p1().iter_mut() {
        info!("some volley");
        *v = volleyball_bundle.velocity;
        *t = volleyball_bundle.transform;
        found = true;
        break
        
    }
    if !found{
        info!("some volley bundle {:?}",volleyball_bundle);
        cmd.spawn_bundle(volleyball_bundle);
    }
}