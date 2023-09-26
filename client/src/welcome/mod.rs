use crate::resources::Global;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use naia_bevy_client::{transport::webrtc, Client};
use naia_bevy_demo_shared::messages::Auth;

use crate::states::MainState;

pub struct WelcomeScreenPlugin;

impl Plugin for WelcomeScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .add_event::<JoinEvent>()
            .init_resource::<UiState>()
            .add_systems(Update, join.run_if(on_event::<JoinEvent>()))
            .add_systems(
                Update,
                name_input_system.run_if(in_state(MainState::Welcome)),
            );
    }
}

#[derive(Default, Resource)]
struct UiState {
    name: String,
    can_join: bool,
}

#[derive(Default, Event)]
struct JoinEvent(String);

impl JoinEvent {
    pub fn player_name(&self) -> String {
        self.0.clone()
    }
}

fn join(mut client: Client, mut join_ev: EventReader<JoinEvent>, mut global: ResMut<Global>) {
    // Process connect sever here?
    // FIXME: I don't want to messing up with these env
    let auth_user_name = env!("AUTH_USER_NAME");
    let auth_user_pass = env!("AUTH_USER_PASS");
    let server_address = env!("SERVER_INIT_ADDRESS");
    //
    client.auth(Auth::new(auth_user_name, auth_user_pass));
    let socket = webrtc::Socket::new(server_address, client.socket_config());
    client.connect(socket);

    for join_data in join_ev.iter() {
        info!("Sending Player Data: {:?}", join_data.player_name());
        // Pass to global is a hack!!!
        global.player_name = join_data.player_name();
    }

    // next_state.set(MainState::Lobby);
}

fn name_input_system(
    mut egui_ctx: EguiContexts,
    mut ui_state: ResMut<UiState>,
    mut join_event: EventWriter<JoinEvent>,
    client: Client,
) {
    egui::CentralPanel::default().show(egui_ctx.ctx_mut(), |ui| {
        ui_state.can_join = !ui_state.name.is_empty();

        ui.add_space(30.);

        ui.vertical_centered(|ui| ui.heading("Tien Len Online"));

        ui.add_space(30.);

        ui.vertical_centered(|ui| {
            ui.add_sized(
                [150.0, 20.0],
                egui::TextEdit::singleline(&mut ui_state.name).hint_text("Enter your name here..."),
            );

            ui.add_space(10.);

            if client.is_connecting() {
                ui.spinner();
                ui.add_space(5.);
                ui.label("Connecting to server...");
            } else if ui
                .add_enabled(ui_state.can_join, egui::Button::new("Join"))
                .clicked()
            {
                join_event.send(JoinEvent(ui_state.name.clone()))
            }
        });

        ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
            ui.add(egui::Hyperlink::from_label_and_url(
                "Developed by Tung To",
                "https://github.com/tungtose",
            ));
        });
        egui::warn_if_debug_build(ui);
    });
}
