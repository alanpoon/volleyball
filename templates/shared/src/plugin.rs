use bevy::prelude::*;
use crate::*;
use crate::systems;
use crate::systems::entity_to_remove;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(SystemLabel)]
pub enum MyLabel {
    Despawn,
}
pub struct SharedPlugin;
impl Plugin for SharedPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app
          .init_resource::<EntityToRemove>()
          .init_resource::<scoreboard::ScoreBoard>()
          .add_system_to_stage(CoreStage::Last,entity_to_remove::remove_entity_system.label(MyLabel::Despawn))
          .add_system(systems::physics::update_state_velocity_physics)
          .add_system(systems::physics::player::remove_move_by_timer)
          .add_system(systems::volleyball_movement)
          .add_startup_system(systems::physics::collider::collider_setup)
          ;
           
    }
  }