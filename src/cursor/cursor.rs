use bevy::{prelude::*, window::*};

use crate::game::game::*;

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Cursor>()
            .add_systems(Update, update_cursor_locking.run_if(in_state(GameState::Playing)))
            .add_systems(Startup, init_cursor_properties);
    }
}

#[derive(Resource,Default)]
pub struct Cursor {
    locked : bool,
}

impl Cursor {
    pub fn invert_lock(&mut self, window : &mut Window) {
        self.locked = !self.locked;
        window.cursor.visible = !self.locked;
        if self.locked {
            let window_width = window.width();
            let window_height = window.height();
            window.cursor.grab_mode = CursorGrabMode::Locked;
            window.set_cursor_position(Some(Vec2::new(window_width/2., window_height/2.)));
        }
        else {
            window.cursor.grab_mode = CursorGrabMode::None;
        }
    }
}

fn init_cursor_properties(
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    if let Ok(mut window) = window_query.get_single_mut() {
        window.cursor.grab_mode = CursorGrabMode::Locked;
        window.cursor.visible = false;
    }
}

fn update_cursor_locking(
    keys: Res<Input<KeyCode>>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
    mut cursor: ResMut<Cursor>
) {
    if keys.just_pressed(KeyCode::Escape) {
        if let Ok(mut window) = window_query.get_single_mut() {
            cursor.invert_lock(&mut window);
        }
    }
}