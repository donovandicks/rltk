use rltk::{Rltk, RGB};

#[derive(PartialEq, Clone, Copy)]
pub enum TileType {
    Wall,
    Floor,
}

/// Retrieves the the tile index for an (x, y) position
///
/// # Args
///
/// * `x` - The x coordinate
/// * `y` - The y coordinate
///
/// # Returns
///
/// * The index of the tile
pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

/// Places walls randomly on the map
///
/// # Args
///
/// * `map` - A mut ref to an existing, initialized map
/// * `width` - The width of the game window
/// * `height` - The height of the game window
///
/// # Returns
///
/// * A mut ref to the map after walls have been placed
fn generate_random_walls(map: &mut Vec<TileType>, width: i32, height: i32) -> &Vec<TileType> {
    let mut rng = rltk::RandomNumberGenerator::new();

    for _ in 0..400 {
        let x = rng.roll_dice(1, width - 1);
        let y = rng.roll_dice(1, height - 1);
        let idx = xy_idx(x, y);
        // Make sure wall is not in the center where the
        // player spawns
        if idx != xy_idx(width / 2, height / 2) {
            map[idx] = TileType::Wall;
        }
    }

    map
}

/// Construct a new map
///
/// # Returns
///
/// * A `Vec<TileType>` representing the map
pub fn new_map(width: i32, height: i32) -> Vec<TileType> {
    let mut map = vec![TileType::Floor; (width * height) as usize];

    // Make boundaries walls
    for x in 0..width {
        map[xy_idx(x, 0)] = TileType::Wall;
        map[xy_idx(x, height - 1)] = TileType::Wall;
    }

    for y in 0..height {
        map[xy_idx(0, y)] = TileType::Wall;
        map[xy_idx(width - 1, y)] = TileType::Wall;
    }

    // Randomly place walls
    generate_random_walls(&mut map, width, height);

    map
}

/// Render the map
///
/// # Args
///
/// * `map` - A slice of tiles
/// * `ctx` - The game context
/// * `width` - The width of the game window
pub fn draw_map(map: &[TileType], ctx: &mut Rltk, width: i32) {
    let mut y = 0;
    let mut x = 0;

    for tile in map.iter() {
        match tile {
            TileType::Floor => {
                ctx.set(
                    x,
                    y,
                    RGB::from_f32(0.5, 0.5, 0.5),
                    RGB::from_f32(0., 0., 0.),
                    rltk::to_cp437('.'),
                );
            }
            TileType::Wall => {
                ctx.set(
                    x,
                    y,
                    RGB::from_f32(0., 1.0, 0.),
                    RGB::from_f32(0., 0., 0.),
                    rltk::to_cp437('#'),
                );
            }
        }

        x += 1;
        if x > width - 1 {
            x = 0;
            y += 1;
        }
    }
}
