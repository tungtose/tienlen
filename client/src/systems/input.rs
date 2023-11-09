use bevy::prelude::{Vec2, Window};

#[allow(dead_code)]
fn window_relative_mouse_position(window: &Window) -> Option<Vec2> {
    let Some(cursor_pos) = window.cursor_position() else {return None};

    let window_size = Vec2 {
        x: window.width(),
        y: window.height(),
    };

    Some(cursor_pos - window_size / 2.0)
}
