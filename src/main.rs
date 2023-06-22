use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_ggrs::{ggrs::SessionBuilder, *};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_matchbox::prelude::*;
use box_game::*;
use card::Card;
use deck::{ActiveCard, Deck};
use input::direction;
use player::{ActivePlayer, Player, PlayerHand, PlayerPosition};
use states::{GameState, MainState};

use crate::ui::ReloadUiEvent;

mod assets;
mod box_game;
mod camera;
mod card;
mod cards;
mod deck;
mod hand;
mod input;
mod manager;
mod player;
mod rank;
mod states;
mod suit;
mod ui;

const FPS: usize = 60;

fn main() {
    let mut app = App::new();

    GGRSPlugin::<GGRSConfig>::new()
        // define frequency of rollback game logic update
        .with_update_frequency(FPS)
        // define system that returns inputs given a player handle, so GGRS can send the inputs
        // around
        .with_input_system(input)
        // register types of components AND resources you want to be rolled back
        // .register_rollback_component::<Card>()
        .register_rollback_component::<Deck>()
        // .register_rollback_component::<Transform>()
        .register_rollback_component::<ActiveCard>()
        .register_rollback_component::<ActivePlayer>()
        .register_rollback_component::<PlayerPosition>()
        .register_rollback_component::<PlayerHand>()
        // .register_rollback_resource::<Deck>()
        // make it happen in the bevy app
        .build(&mut app);

    app.add_state::<MainState>()
        .add_state::<GameState>()
        // .insert_resource(WinitSettings::desktop_app())
        .add_plugins(DefaultPlugins)
        // Show fps
        // .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(assets::AssetPlugin)
        .add_plugin(ui::UiPlugin)
        .add_plugin(player::PlayerPlugin)
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(deck::DeckPlugin)
        .add_startup_system(camera::setup)
        .add_system(lobby_system.run_if(in_state(MainState::Lobby)))
        // .add_system(wait_for_players.run_if(in_state(MainState::Lobby)))
        .add_system(lobby_cleanup.in_schedule(OnExit(MainState::Lobby)))
        // .add_system(spawn_player.in_schedule(OnEnter(MainState::Game)))
        // .add_system(move_players.in_schedule(GGRSSchedule))
        // .add_system(log_ggrs_events.run_if(in_state(MainState::Lobby)))
        .add_system(log_ggrs_events.run_if(in_state(MainState::Game)))
        .add_systems((lobby_startup, start_matchbox_socket).in_schedule(OnEnter(MainState::Lobby)))
        .run();
}

#[derive(Debug)]
struct GGRSConfig;

impl ggrs::Config for GGRSConfig {
    type Input = u8;

    type State = u8;

    type Address = PeerId;
}

#[derive(Component, Reflect, Default, Clone, Copy)]
pub struct MoveDir(pub Vec2);

fn spawn_player(mut commands: Commands, mut rip: ResMut<RollbackIdProvider>) {
    info!("spawning players");

    commands.spawn((
        Player { handle: 0 },
        rip.next(),
        MoveDir(Vec2::X),
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(-2., 0., 100.)),
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(Vec2::new(1., 1.)),
                ..default()
            },
            ..default()
        },
    ));
}

fn start_matchbox_socket(mut commands: Commands) {
    info!("Start matchbox socket !!!");
    let room_url = "ws://127.0.0.1:3536/thirteen?next=4";
    // let room_id = match &args.room {
    //     Some(id) => id.clone(),
    //     None => format!("bevy_ggrs?next={}", &args.players),
    // };
    //
    // let room_url = format!("{}/{}", &args.matchbox, room_id);
    info!("connecting to matchbox server: {:?}", room_url);

    commands.insert_resource(MatchboxSocket::new_reliable(room_url));
}

// Marker components for UI
#[derive(Component)]
struct LobbyText;
#[derive(Component)]
struct LobbyUI;

fn lobby_startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // All this is just for spawning centered text.
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexEnd,
                ..default()
            },
            background_color: Color::rgb(0.43, 0.41, 0.38).into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(TextBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    text: Text::from_section(
                        "Entering lobby...",
                        TextStyle {
                            font: asset_server.load("fonts/FiraCode-Bold.otf"),
                            font_size: 48.,
                            color: Color::BLACK,
                        },
                    ),
                    ..default()
                })
                .insert(LobbyText);
        })
        .insert(LobbyUI);
}

fn lobby_cleanup(query: Query<Entity, With<LobbyUI>>, mut commands: Commands) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

const MAX_PLAYER: usize = 2;

fn lobby_system(
    mut app_state: ResMut<NextState<MainState>>,
    mut socket: ResMut<MatchboxSocket<SingleChannel>>,
    mut commands: Commands,
    mut query: Query<&mut Text, With<LobbyText>>,
) {
    // regularly call update_peers to update the list of connected peers
    for (peer, new_state) in socket.update_peers() {
        // you can also handle the specific dis(connections) as they occur:
        match new_state {
            PeerState::Connected => info!("peer {peer:?} connected"),
            PeerState::Disconnected => info!("peer {peer:?} disconnected"),
        }
    }

    let connected_peers = socket.connected_peers().count();
    let remaining = MAX_PLAYER - (connected_peers + 1);
    query.single_mut().sections[0].value = format!("Waiting for {remaining} more player(s)",);
    if remaining > 0 {
        return;
    }

    info!("All peers have joined, going in-game");

    // extract final player list
    let players = socket.players();

    let max_prediction = 12;

    // create a GGRS P2P session
    let mut sess_build = SessionBuilder::<GGRSConfig>::new()
        .with_num_players(MAX_PLAYER)
        .with_max_prediction_window(max_prediction)
        .with_input_delay(2)
        .with_fps(FPS)
        .expect("invalid fps");

    for (i, player) in players.into_iter().enumerate() {
        sess_build = sess_build
            .add_player(player, i)
            .expect("failed to add player");
    }

    let channel = socket.take_channel(0).unwrap();

    // start the GGRS session
    let sess = sess_build
        .start_p2p_session(channel)
        .expect("failed to start session");

    commands.insert_resource(Session::P2PSession(sess));

    // transition to in-game state
    app_state.set(MainState::Game);
}

fn log_ggrs_events(mut session: ResMut<Session<GGRSConfig>>) {
    match session.as_mut() {
        Session::P2PSession(s) => {
            for event in s.events() {
                info!("GGRS Event: {:?}", event);
            }
        }
        _ => panic!("This example focuses on p2p."),
    }
}
