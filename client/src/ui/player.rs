use std::collections::HashMap;

use bevy::prelude::*;
use naia_bevy_demo_shared::components::card::Card;

use crate::{components::LocalPlayer, game::ActiveCard};

use super::{DrawPlayer, UiAssets};

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
pub struct CardButton(Entity);

#[derive(Component)]
pub struct CardHandleImageMap {
    pub map: HashMap<Entity, Handle<Image>>,
}

fn create_hand_container(commands: &mut Commands, pos: Vec2) -> Entity {
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
                ..Default::default()
            },
        ))
        .id();

    hand_container
}

pub fn get_card(
    commands: &mut Commands,
    size: Size,
    // margin: UiRect,
    is_active: bool,
    image: &Handle<Image>,
) -> Entity {
    let mut margin = UiRect::all(Val::Px(CARD_MARGIN));

    if is_active {
        margin.bottom = Val::Px(CARD_SELECT);
    }

    commands
        .spawn((
            CardUi,
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

pub fn card_click(
    mut interactions: Query<(&Interaction, &CardButton), Changed<Interaction>>,
    mut active_card_q: Query<&mut ActiveCard>,
    mut draw_player_ev: EventWriter<DrawPlayer>,
) {
    for (interaction, button) in interactions.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                info!("clicking on: {:?}", button.0);
                let mut active = active_card_q.get_mut(button.0).unwrap();

                active.0 = !active.0;

                info!("Start DRAWING NOW!!!");
                draw_player_ev.send(DrawPlayer);
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
    card_entity_q: Query<Entity, With<Card>>,
    card_q: Query<(&Card, &ActiveCard)>,
) {
    clear_hand_ui(&mut commands, &hand_container_query);

    let hand_container = create_hand_container(&mut commands, Vec2::from_array([0., 300.]));

    for card_entity in card_entity_q.iter() {
        let (card, active_card) = card_q.get(card_entity).unwrap();
        let handle = card_assets.cards.get(&card.name()).unwrap();

        let card_ui = get_card(
            &mut commands,
            Size::new(Val::Px(CARD_WIDTH), Val::Px(CARD_HEIGHT)),
            active_card.0,
            handle,
        );

        commands.entity(card_ui).insert(CardButton(card_entity));

        commands.entity(hand_container).add_child(card_ui);
    }
}
