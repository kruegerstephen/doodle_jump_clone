use bevy::prelude::*;


pub struct EventPipelinePlugin;

impl Plugin for EventPipelinePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RestartLevelEvent>();
    }
}


#[derive(Event, Default)]
pub struct RestartLevelEvent;





