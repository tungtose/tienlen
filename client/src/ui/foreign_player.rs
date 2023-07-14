use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use naia_bevy_demo_shared::components::Player;

use crate::components::LocalPlayer;

#[derive(Component)]
pub struct ForeignPlayer;

#[derive(Component)]
pub struct CircleSetting {
    angle: f32,
    center: Vec2,
    radii: Vec2,
    rotate: f32,
    pos: Vec2,
}

impl Default for CircleSetting {
    fn default() -> Self {
        Self {
            pos: Vec2::new(-300., 0.),
            angle: 0.0,
            center: Vec2::new(-300., 0.),
            radii: Vec2::new(30., 30.),
            rotate: 0.,
        }
    }
}

impl CircleSetting {
    pub fn new(pos: Vec2) -> Self {
        Self {
            pos,
            angle: 0.0,
            center: pos,
            radii: Vec2::new(30., 30.),
            rotate: 0.,
        }
    }

    pub fn validate_arc(&mut self) {
        if self.angle < f32::to_radians(-360.) || self.angle > f32::to_radians(360.) {
            self.angle = f32::to_radians(0.);
        }
    }

    pub fn angle_degree(&self) -> f32 {
        f32::to_degrees(self.angle)
    }

    pub fn update_angle(&mut self, time_out: f32, delta: f32) {
        self.angle = f32::to_radians(self.angle_degree() - (360. / time_out) * delta);

        self.validate_arc();
    }
}

const TIME_OUT: f32 = 10.;

pub fn circle_cooldown_update(
    mut foreign_player_q: Query<(&mut CircleSetting, &mut Path), With<ForeignPlayer>>,
    time: Res<Time>,
) {
    let delta = time.delta_seconds();

    for (mut cir_setting, mut path) in foreign_player_q.iter_mut() {
        let mut path_builder = PathBuilder::new();

        cir_setting.update_angle(TIME_OUT, delta);

        path_builder.move_to(cir_setting.pos);
        path_builder.line_to(Vec2::new(
            cir_setting.pos.x,
            cir_setting.radii.y + cir_setting.pos.y,
        ));
        path_builder.arc(
            cir_setting.center,
            cir_setting.radii,
            cir_setting.angle,
            cir_setting.rotate,
        );

        path_builder.close();
        let new_path = path_builder.build();

        *path = new_path;
    }
}

pub fn spawn_foreign_player(
    mut commands: Commands,
    players_q: Query<&Player, Without<LocalPlayer>>,
    local_player_q: Query<&Player, With<LocalPlayer>>,
) {
    let l_player = local_player_q.get_single().unwrap();
    let l_player_pos = *l_player.pos.clone() as i32;

    for player in players_q.iter() {
        let player_pos = *player.pos.clone() as i32;

        // FIXME: Find the way to clean this mess
        let mut pos: Vec2 = Vec2::new(0., 0.);
        match l_player_pos {
            0 => {
                if player_pos == 1 {
                    pos = Vec2::new(-335., 35.);
                }
                if player_pos == 2 {
                    pos = Vec2::new(0., 210.);
                }
                if player_pos == 3 {
                    pos = Vec2::new(335., 35.);
                }
            }
            1 => {
                if player_pos == 2 {
                    pos = Vec2::new(-335., 35.);
                }
                if player_pos == 3 {
                    pos = Vec2::new(0., 210.);
                }
                if player_pos == 0 {
                    pos = Vec2::new(335., 35.);
                }
            }
            2 => {
                if player_pos == 3 {
                    pos = Vec2::new(-335., 35.);
                }
                if player_pos == 0 {
                    pos = Vec2::new(0., 210.);
                }
                if player_pos == 1 {
                    pos = Vec2::new(335., 35.);
                }
            }
            3 => {
                if player_pos == 0 {
                    pos = Vec2::new(-335., 35.);
                }
                if player_pos == 1 {
                    pos = Vec2::new(0., 210.);
                }
                if player_pos == 2 {
                    pos = Vec2::new(335., 35.);
                }
            }
            _ => {}
        }

        let cir_setting = CircleSetting::new(pos);
        let mut path_builder = PathBuilder::new();

        path_builder.move_to(cir_setting.pos);
        path_builder.line_to(Vec2::new(
            cir_setting.pos.x,
            cir_setting.radii.y + cir_setting.pos.y,
        ));
        path_builder.arc(
            cir_setting.center,
            cir_setting.radii,
            cir_setting.angle,
            cir_setting.rotate,
        );

        path_builder.close();
        let path = path_builder.build();

        let color = Color::with_a(Color::GRAY, 0.07);

        commands.spawn((
            cir_setting,
            ForeignPlayer,
            ShapeBundle {
                path,
                // transform: Transform::from_xyz(0., 0., 0.),
                ..default()
            },
            Stroke::new(Color::RED, 5.),
            Fill::color(color),
        ));
    }
}
