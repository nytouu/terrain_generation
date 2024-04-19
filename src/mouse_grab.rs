use bevy::{prelude::*, window::CursorGrabMode};

pub struct MouseGrabPlugin;

impl Plugin for MouseGrabPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, grab_mouse);
    }
}

fn grab_mouse(
    mut windows: Query<&mut Window>,
    key: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    let mut window = windows.single_mut();

    if mouse.just_pressed(MouseButton::Middle) {
        window.cursor.visible = false;
        window.cursor.grab_mode = CursorGrabMode::Locked;
    }

    if key.just_pressed(KeyCode::Escape) {
        window.cursor.visible = true;
        window.cursor.grab_mode = CursorGrabMode::None;
    }
}
