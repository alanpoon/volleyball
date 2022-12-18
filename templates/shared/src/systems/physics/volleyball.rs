use bevy_rapier2d::prelude::*;
use bevy::prelude::*;
use bevy_rapier2d::math::Vect;
use crate::*;

pub fn volleyball_setup(mut cmd:Commands){
    cmd.spawn_bundle(TransformBundle::from(Transform::from_xyz(
        -940.0,
        -300.0,
        3.0,
    )))
    .insert(Collider::ball(50.0));
}
pub fn volleyball_movement(mut volley_query:Query<(&mut VolleyBall,&Transform,&mut Velocity)>,
ball_query:Query<(&BallId,&Transform)>,
out_of_bound_query:Query<(&OutOfBound,&Transform)>,
mut scoreboard:ResMut<scoreboard::ScoreBoard>
){
    for (mut vb,v_t,mut v_v) in volley_query.iter_mut(){
        for (b,b_t) in ball_query.iter(){
            if v_t.translation.distance(b_t.translation)<140.0{
                let mut is_left = false;
                if v_t.translation.x <0.0{
                    v_v.linvel.x = 400.0;
                    is_left = true;
                }else{
                    v_v.linvel.x = -400.0;
                    is_left = false;
                }
                vb.last_touch = Some(LastTouch(b.0,is_left));
                if v_t.translation.y <600.0{
                    v_v.linvel.y = 600.0;
                }else{
                    v_v.linvel.y = -600.0;
                }
                
            }
        }
        for (out_of_bound,o_t) in out_of_bound_query.iter(){
            if v_t.translation.distance(o_t.translation)<75.0{
                if let Some(LastTouch(ball_id,is_left)) = vb.last_touch{
                    if is_left{
                        scoreboard.b+=1;
                    }else{
                        scoreboard.a+=1;
                    }
                }
            }
        }
    }
    
}