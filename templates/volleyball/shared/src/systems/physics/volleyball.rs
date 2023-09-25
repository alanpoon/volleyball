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
pub fn volleyball_movement(
    mut cmd:Commands,
    mut volley_query:Query<(&mut VolleyBall,&mut Transform,&mut Velocity),(Without<BallId>,Without<OutOfBound>)>,
    ball_query:Query<(&BallId,&Transform)>,
    out_of_bound_query:Query<(&OutOfBound,&Transform)>,
    mut scoreboard:ResMut<scoreboard::ScoreBoard>
){
    for (mut vb,mut v_t,mut v_v) in volley_query.iter_mut(){
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
        // for (out_of_bound,o_t) in out_of_bound_query.iter(){
        //     if v_t.translation.distance(o_t.translation)<120.0{
        //         info!("out_of_bound lasttouch{:?}",vb.last_touch);
        //         if let Some(LastTouch(ball_id,is_left)) = vb.last_touch{
        //             if is_left{
        //                 v_t.translation.x = 940.0;
        //                 v_t.translation.y = 200.0;
        //                 scoreboard.b+=1;
        //                 cmd.spawn().insert(ScoreAnimation(false,Timer::from_seconds(3.0,false)));
        //             }else{
        //                 v_t.translation.x = -940.0;
        //                 v_t.translation.y = 200.0;
        //                 scoreboard.a+=1;
        //                 cmd.spawn().insert(ScoreAnimation(true,Timer::from_seconds(3.0,false)));
        //             }
        //             vb.last_touch = None;
        //         }
        //     }
        // }
    }
}
pub fn volley_collision(mut cmd:Commands,mut collision_events: EventReader<CollisionEvent>,
    mut volley_query:Query<(&mut VolleyBall, &mut Transform),With<VolleyBall>>,
    out_of_bound_query:Query<(Entity,&OutOfBound)>,
    mut scoreboard:ResMut<scoreboard::ScoreBoard>){
    for collision_event in collision_events.iter() {
        match collision_event{
            CollisionEvent::Started(e1,e2,_)=>{
                info!("Received collision event: e1{:?}, e2{:?}", e1,e2);
                for (entity,_) in out_of_bound_query.iter(){
                    info!("out_of_bound_query entity entity{:?}", entity);
                    if e2==&entity{
                        info!("out_of_bound_query matched entity{:?}", entity);
                        if let Ok((mut vb,mut v_t)) = volley_query.get_mut(*e1){
                            if let Some(LastTouch(ball_id,is_left)) = vb.last_touch{
                                if is_left{
                                    v_t.translation.x = 940.0;
                                    v_t.translation.y = 200.0;
                                    scoreboard.b+=1;
                                    cmd.spawn().insert(ScoreAnimation(false,Timer::from_seconds(3.0,false)));
                                }else{
                                    v_t.translation.x = -940.0;
                                    v_t.translation.y = 200.0;
                                    scoreboard.a+=1;
                                    cmd.spawn().insert(ScoreAnimation(true,Timer::from_seconds(3.0,false)));
                                }
                                vb.last_touch = None;
                            }
                        }
                        break;
                    }
                }
             
            },
            CollisionEvent::Stopped(e1,e2,_)=>{}
        }
        
    }
}