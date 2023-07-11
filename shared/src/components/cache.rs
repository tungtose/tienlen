use bevy_ecs::prelude::Component;

#[derive(Component)]
pub struct Cache {
    current_active_player: usize,
    total_player: usize,
}
