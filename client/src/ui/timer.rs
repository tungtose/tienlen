use bevy::prelude::*;
use naia_bevy_demo_shared::components::timer::Counter;

use crate::resources::Global;

use super::UiAssets;

#[derive(Component)]
pub struct LocalTimer(pub i32);

#[derive(Component)]
pub struct TimerContainer;

#[derive(Component)]
pub struct SkipTurnTimerText;

const TIMER_CONTAINER_HEIGHT: f32 = 48.;

fn create_timer_container(commands: &mut Commands) -> Entity {
    let timer_container = commands
        .spawn((
            TimerContainer,
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect::bottom(Val::Px(0.)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
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

pub fn init_counter(mut commands: Commands, res: Res<UiAssets>) {
    let container = create_timer_container(&mut commands);

    let timer_entity = commands
        .spawn((
            SkipTurnTimerText,
            TextBundle::from_section(
                "0".to_string(),
                TextStyle {
                    font: res.font.clone(),
                    font_size: 32.0,
                    color: Color::RED,
                },
            ),
        ))
        .id();

    commands.entity(container).add_child(timer_entity);
}

pub fn update_counter(
    global: ResMut<Global>,
    res: Res<UiAssets>,
    mut text_q: Query<&mut Text, With<SkipTurnTimerText>>,
) {
    let mut text_counter = text_q.get_single_mut().unwrap();

    let text_style = TextStyle {
        font: res.font.clone(),
        font_size: 32.0,
        color: Color::RED,
    };

    *text_counter = Text::from_section(&global.game.timer, text_style);
}
