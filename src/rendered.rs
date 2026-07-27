fn render3d(framebuffer: &mut Framebuffer, player: &Player) {
    let maze = load_maze("./maze.txt");
    let block_size = 100;
    let num_rays = framebuffer.width;

    let hw = framebuffer.width as f32 / 2.0;   // precalculated half width
    let hh = framebuffer.height as f32 / 2.0;  // precalculated half height

    framebuffer.set_current_color(0xFFFFFF);

    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32; // current ray divided by total rays
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        let intersect = cast_ray(framebuffer, &maze, &player, a, block_size, false);

        // Calculate the height of the stake
        let distance_to_wall = ???; // how far is this wall from the player
        let distance_to_projection_plane = ???; // how far is the "player" from the "camera"
        // this ratio doesn't really matter as long as it is a function of distance
        let stake_height = (hh / distance_to_wall) * distance_to_projection_plane;

        // Calculate the position to draw the stake
        let stake_top = (hh - (stake_height / 2.0)) as usize;
        let stake_bottom = (hh + (stake_height / 2.0)) as usize;

        // Draw the stake directly in the framebuffer
        for y in stake_top..stake_bottom {
            framebuffer.point(i, y); // Assuming white color for the stake
        }
    }
}