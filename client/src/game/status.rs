use std::time::Duration;

use bevy::prelude::*;

use crate::{states::MainState, ui::UiAssets};

pub struct StatusPlugin;

impl Plugin for StatusPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DrawStatus>()
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (draw_status, delete_status).run_if(in_state(MainState::Game)),
            );
    }
}

#[derive(Default, Event)]
pub struct DrawStatus(pub String);

#[derive(Component)]
pub struct StatusContainer;

#[derive(Component)]
pub struct CounterConfig {
    timer: Timer,
}

#[derive(Component)]
pub struct Table;

pub fn setup(mut commands: Commands, res: Res<UiAssets>) {}

pub fn delete_status(
    mut commands: Commands,
    mut counter_q: Query<(Entity, &mut CounterConfig)>,
    status_container_q: Query<Entity, With<StatusContainer>>,
    time: Res<Time>,
) {
    for (entity, mut counter) in counter_q.iter_mut() {
        counter.timer.tick(time.delta());

        if counter.timer.finished() {
            commands.entity(entity).despawn();

            let status_container = status_container_q.get_single().unwrap();

            commands.entity(status_container).despawn_descendants();
        }
    }
}

pub fn draw_status(
    mut commands: Commands,
    mut status_ev: EventReader<DrawStatus>,
    status_container_q: Query<Entity, With<StatusContainer>>,
    res: Res<UiAssets>,
) {
    let status_container = status_container_q.get_single().unwrap();

    for d_status in status_ev.iter() {
        let msg = d_status.0.clone();

        let status_text = commands
            .spawn(TextBundle::from_section(
                msg,
                TextStyle {
                    font: res.font.clone(),
                    font_size: 16.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ))
            .id();

        commands.entity(status_container).add_child(status_text);
        commands.spawn(CounterConfig {
            timer: Timer::new(Duration::from_secs(3), TimerMode::Once),
        });
    }
}
