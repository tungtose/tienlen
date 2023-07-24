use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

use crate::states::{self, MainState};

pub struct WelcomeScreenPlugin;

impl Plugin for WelcomeScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin)
            .add_event::<JoinEvent>()
            .init_resource::<UiState>()
            .add_system(join.run_if(on_event::<JoinEvent>()))
            .add_system(name_input_system.run_if(in_state(states::MainState::Welcome)));
    }
}

#[derive(Default, Resource)]
struct UiState {
    name: String,
    can_join: bool,
}

#[derive(Default)]
struct JoinEvent(pub String);

fn join(mut next_state: ResMut<NextState<MainState>>) {
    next_state.set(MainState::Lobby);
}

fn name_input_system(
    mut egui_ctx: EguiContexts,
    mut ui_state: ResMut<UiState>,
    mut join_event: EventWriter<JoinEvent>,
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

            if ui
                .add_enabled(ui_state.can_join, egui::Button::new("Join"))
                .clicked()
            {
                info!("Clicked btn");
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
