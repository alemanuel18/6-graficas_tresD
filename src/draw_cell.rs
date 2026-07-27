fn draw_cell(
    framebuffer: &mut Framebuffer,
    xo: usize,
    yo: usize,
    block_size: usize,
    cell: char,
) {
    // pinten un rectangulo de diferente color segun cada char
}

pub fn render_maze(
    framebuffer: &mut Framebuffer,
    maze: &Vec<Vec<char>>,
    block_size: usize,
) {
    for (row_index, row) in maze.iter().enumerate() {
        for (col_index, &cell) in row.iter().enumerate() {
            let xo = col_index * block_size;
            let yo = row_index * block_size;
            // llamen a su draw cell
        }
    }
}