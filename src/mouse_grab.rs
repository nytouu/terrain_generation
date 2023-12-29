use bevy::{prelude::*, window::CursorGrabMode};

pub struct MouseGrabPlugin;

impl Plugin for MouseGrabPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, grab_mouse);
    }
}

fn grab_mouse(
    mut windows: Query<&mut Window>,
    key: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
) {
    let mut window = windows.single_mut();

    if mouse.just_pressed(MouseButton::Left) {
        window.cursor.visible = false;
        window.cursor.grab_mode = CursorGrabMode::Locked;
    }

    if key.just_pressed(KeyCode::Escape) {
        window.cursor.visible = true;
        window.cursor.grab_mode = CursorGrabMode::None;
    }
}
