use std::collections::HashMap;

use bevy::prelude::*;

use crate::game::{ActiveCards, LocalPlayerCards, SelectCardEvent};

use super::{table::TableContainer, UiAssets};

const DECK_HEIGHT: f32 = 50.;
const CARD_WIDTH: f32 = 32.;
const CARD_HEIGHT: f32 = 48.;
const CARD_MARGIN: f32 = 2.;
const CARD_SELECT: f32 = 24.;

#[derive(Component)]
pub struct HandContainer;

#[derive(Component)]
pub struct CardUi;

#[derive(Component)]
pub struct CardButton(usize);

#[derive(Component)]
pub struct CardHandleImageMap {
    pub map: HashMap<Entity, Handle<Image>>,
}

fn create_hand_container(commands: &mut Commands, active: bool) -> Entity {
    let background_color = {
        if active {
            BackgroundColor(Color::DARK_GREEN)
        } else {
            BackgroundColor(Color::NONE)
        }
    };
    let hand_container = commands
        .spawn((
            HandContainer,
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(0.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.),
                    height: Val::Px(DECK_HEIGHT),
                    ..Default::default()
                },
                background_color,
                ..Default::default()
            },
        ))
        .id();

    hand_container
}

pub fn get_card(commands: &mut Commands, is_active: bool, image: &Handle<Image>) -> Entity {
    let mut margin = UiRect::all(Val::Px(CARD_MARGIN));

    if is_active {
        margin.bottom = Val::Px(CARD_SELECT);
    }

    commands
        .spawn((
            CardUi,
            ButtonBundle {
                style: Style {
                    width: Val::Px(CARD_WIDTH),
                    height: Val::Px(CARD_HEIGHT),
                    margin,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                image: UiImage::new(image.clone()),
                ..Default::default()
            },
        ))
        .id()
}

pub fn card_click(
    mut interactions: Query<(&Interaction, &CardButton), Changed<Interaction>>,
    mut select_card_ev: EventWriter<SelectCardEvent>,
) {
    for (interaction, button) in interactions.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                select_card_ev.send(SelectCardEvent(button.0));
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

fn clear_hand_ui(commands: &mut Commands, query: &Query<Entity, With<HandContainer>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn draw_player(
    mut commands: Commands,
    card_assets: Res<UiAssets>,
    hand_container_query: Query<Entity, With<HandContainer>>,
    table_container_query: Query<Entity, With<TableContainer>>,
    local_player_cards_q: Query<&LocalPlayerCards>,
    active_cards_q: Query<&ActiveCards>,
) {
    // let Ok(table_container_entity) = table_container_query.get_single() else {
    //     return;
    // };
    //
    // clear_hand_ui(&mut commands, &hand_container_query);
    //
    // let hand_container = create_hand_container(&mut commands, false);
    //
    // let Ok(active_cards) = active_cards_q.get_single() else {
    //     return;
    // };
    //
    // let Ok(player_cards) = local_player_cards_q.get_single() else {
    //     return;
    // };
    //
    // for (card_entity, card) in player_cards.0.iter() {
    //     let handle = card_assets.cards.get(&card.name()).unwrap();
    //
    //     let is_active = active_cards.is_active(card_entity);
    //
    //     let card_ui = get_card(&mut commands, is_active, handle);
    //
    //     commands.entity(card_ui).insert(CardButton(*card_entity));
    //
    //     commands.entity(hand_container).add_child(card_ui);
    // }
    //
    // commands
    //     .entity(table_container_entity)
    //     .add_child(hand_container);
}
