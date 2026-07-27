pub fn process_events(window: &Window, player: &mut Player) {
    const MOVE_SPEED = 10.0;
    const ROTATION_SPEED = PI / 10.0;

    if window.is_key_down(Key::Left) {
        // rotate the view range to the left
    }

    if window.is_key_down(Key::Right) {
        // rotate the view range to the right
    }

    if window.is_key_down(Key::Up) {
        // increase player position in x and y in the direction of view
    }

    if window.is_key_down(Key::Down) {
        // decrease player position in x and y in the direction of view
    }
}