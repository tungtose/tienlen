use crate::{
    card::Card,
    deck::ActiveCard,
    player::{ActivePlayer, Player, PlayerEvent, PlayerEventKind, PlayerHand, PlayerPosition},
};
use bevy::prelude::*;

use super::UiAssets;

const DECK_HEIGHT: f32 = 50.;
const CARD_WIDTH: f32 = 32.;
const CARD_HEIGHT: f32 = 48.;
const CARD_MARGIN: f32 = 2.;
const CARD_SELECT: f32 = 24.;

#[derive(Component)]
pub struct HandContainer;

fn create_hand_container(
    commands: &mut Commands,
    pos: Vec2,
    active_player: &Option<&ActivePlayer>,
) -> Entity {
    let background = if active_player.is_some() {
        BackgroundColor(Color::NAVY)
    } else {
        BackgroundColor(Color::NONE)
    };

    let hand_container = commands
        .spawn((
            HandContainer,
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect::bottom(Val::Px(pos.y)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    size: Size::new(Val::Percent(100.), Val::Px(DECK_HEIGHT)),
                    ..Default::default()
                },
                background_color: background,
                ..Default::default()
            },
        ))
        .id();

    hand_container
}

#[derive(Component)]
pub struct ClickableButton;

#[derive(Component)]
pub struct CardButton(Entity, bool, PlayerPosition);

pub fn get_button(
    commands: &mut Commands,
    size: Size,
    margin: UiRect,
    image: &Handle<Image>,
) -> Entity {
    commands
        .spawn((
            ClickableButton,
            ButtonBundle {
                style: Style {
                    size,
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

pub fn draw_hand(
    mut commands: &mut Commands,
    card_assets: &Res<UiAssets>,
    hand_container: Entity,
    hand: &PlayerHand,
    card_query: &Query<(&Card, &ActiveCard)>,
    player_pos: &PlayerPosition,
) {
    // info!("drawing hand!!!");
    // println!("cards: {:?}", hand.active_cards);
    for card_id in hand.cards.as_slice() {
        let (card, active_card) = card_query.get(*card_id).unwrap();
        let handle = card_assets.cards.get(&card.name()).unwrap();
        let mut margin = UiRect::all(Val::Px(CARD_MARGIN));

        if active_card.0 {
            margin.bottom = Val::Px(CARD_SELECT);
        }

        let button = get_button(
            &mut commands,
            Size::new(Val::Px(CARD_WIDTH), Val::Px(CARD_HEIGHT)),
            margin,
            handle,
        );

        commands
            .entity(button)
            .insert(CardButton(*card_id, false, *player_pos));

        commands.entity(hand_container).add_child(button);
    }
}

pub fn card_click(
    mut interactions: Query<(&Interaction, &mut CardButton), Changed<Interaction>>,
    mut ev_hand: EventWriter<crate::player::PlayerEvent>,
) {
    for (interaction, mut button) in interactions.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                // info!("clicking");
                button.1 = true;
                ev_hand.send(PlayerEvent(PlayerEventKind::SelectCard(button.0, button.2)));
            }
            Interaction::Hovered => {
                // info!("hovering");
            }
            Interaction::None => button.1 = false,
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
    hand_query: Query<(
        &mut PlayerHand,
        &Player,
        &PlayerPosition,
        Option<&ActivePlayer>,
    )>,
    card_query: Query<(&Card, &ActiveCard)>,
    hand_container_query: Query<Entity, With<HandContainer>>,
) {
    clear_hand_ui(&mut commands, &hand_container_query);
    info!("Draw player!");
    for (player_hand, _player, pos, active_player) in hand_query.iter() {
        match pos {
            PlayerPosition(0) => {
                info!("hand 1: {:?}", player_hand.cards);
                let hand_container = create_hand_container(
                    &mut commands,
                    Vec2::from_array([0., 300.]),
                    &active_player,
                );

                draw_hand(
                    &mut commands,
                    &card_assets,
                    hand_container,
                    player_hand,
                    &card_query,
                    &PlayerPosition(0),
                );
            }
            PlayerPosition(1) => {
                info!("hand 2: {:?}", player_hand.cards);
                let hand_container = create_hand_container(
                    &mut commands,
                    Vec2::from_array([0., 230.]),
                    &active_player,
                );

                draw_hand(
                    &mut commands,
                    &card_assets,
                    hand_container,
                    player_hand,
                    &card_query,
                    &PlayerPosition(1),
                );
            }
            PlayerPosition(2) => {
                let hand_container = create_hand_container(
                    &mut commands,
                    Vec2::from_array([0., 210.]),
                    &active_player,
                );

                draw_hand(
                    &mut commands,
                    &card_assets,
                    hand_container,
                    player_hand,
                    &card_query,
                    &PlayerPosition(2),
                );
            }
            PlayerPosition(3) => {
                let hand_container = create_hand_container(
                    &mut commands,
                    Vec2::from_array([0., 140.]),
                    &active_player,
                );

                draw_hand(
                    &mut commands,
                    &card_assets,
                    hand_container,
                    player_hand,
                    &card_query,
                    &PlayerPosition(3),
                );
            }
            _ => {}
        }
    }
}
