use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use shared::*;
pub fn _fn(
    cmd: &mut Commands,
    set: &mut ParamSet<(Query<(Entity, &BallId, &mut Transform, &mut Velocity), With<BallId>>,
    Query<(Entity, &VolleyBall, &mut Transform, &mut Velocity), With<VolleyBall>>,
)>,
    ball_id: BallId,
    tv: Vec2,
) {
    //let mut to_insert_move_timer =None;
    for (entity, qball_id, _t, mut v) in set.p0().iter_mut() {
        if ball_id == *qball_id {
            update::target_velocity::velocity(&mut v, tv.clone());
            //to_insert_move_timer = Some(entity);
            break;
        }
    }
    // if let Some(e) = to_insert_move_timer{
    //     cmd.entity(e).insert(MoveTimer(Timer::from_seconds(1.0,false)));
    // }
}
