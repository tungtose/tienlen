use bevy::prelude::*;
use naia_bevy_demo_shared::components::timer::Counter;

use super::UiAssets;

#[derive(Component)]
pub struct LocalTimer(pub i32);

#[derive(Component)]
pub struct TimerContainer;

#[derive(Component)]
pub struct SkipTurnTimerText;

const TIMER_CONTAINER_HEIGHT: f32 = 48.;

fn create_timer_container(commands: &mut Commands, pos: (f32, f32)) -> Entity {
    let timer_container = commands
        .spawn((
            TimerContainer,
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect::top(Val::Px(pos.1)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    size: Size::new(Val::Percent(100.), Val::Px(TIMER_CONTAINER_HEIGHT)),
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .id();

    timer_container
}

fn clear_counter(commands: &mut Commands, query: &Query<Entity, With<TimerContainer>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn draw_counter(
    mut commands: Commands,
    timer_q: Query<&Counter>,
    res: Res<UiAssets>,
    timer_container_query: Query<Entity, With<TimerContainer>>,
) {
    clear_counter(&mut commands, &timer_container_query);

    let Ok(server_timer) = timer_q.get_single() else {
        info!("No timer yet!");
        return;
    };

    let container = create_timer_container(&mut commands, (0., 20.));

    let timer_string = server_timer.as_string();

    let timer_entity = commands
        .spawn((
            SkipTurnTimerText,
            TextBundle::from_section(
                timer_string,
                TextStyle {
                    font: res.font.clone(),
                    font_size: 32.0,
                    color: Color::RED,
                },
            ),
        ))
        .id();

    commands.entity(container).add_child(timer_entity);

    // info!("Counter: {:?}", *server_timer.counter);
}
