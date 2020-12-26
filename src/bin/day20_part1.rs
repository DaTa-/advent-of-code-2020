fn main() {
    let input = String::from_utf8(std::fs::read("input/day20").unwrap()).unwrap();
    let tile_len = input.split('\n').skip(1).next().unwrap().len();
    let tiles: Vec<(u16, Vec<bool>)> = input
        .split_terminator("\n\n")
        .map(|tile| {
            let mut tile = tile.split('\n');
            let id = tile
                .next()
                .unwrap()
                .strip_prefix("Tile ")
                .unwrap()
                .strip_suffix(':')
                .unwrap()
                .parse()
                .unwrap();
            let tile = (0..tile_len * tile_len)
                .zip(tile.flat_map(|line| line.chars()))
                .map(|(_, c)| c == '#')
                .collect();
            (id, tile)
        })
        .collect();

    let rotate = |[x, y]: [usize; 2]| -> [usize; 2] { [y, x] };
    let flip = |mut coords: [usize; 2], dim: usize| -> [usize; 2] {
        coords[dim] = tile_len - 1 - coords[dim];
        coords
    };
    let transform_count = 2usize.pow(3);
    let rotation = 1 << 0;
    let flip0 = 1 << 1;
    let flip1 = 1 << 2;
    let apply_transforms = |mut coords: [usize; 2], transform: usize| -> [usize; 2] {
        if transform & rotation != 0 {
            coords = rotate(coords);
        }
        if transform & flip0 != 0 {
            coords = flip(coords, 0);
        }
        if transform & flip1 != 0 {
            coords = flip(coords, 1);
        }
        coords
    };
    let side_transforms = [0, flip1, rotation, rotation | flip0];
    let sides_count = side_transforms.len();
    let to_index = |[x, y]: [usize; 2]| -> usize { x + tile_len * y };

    let answer: u64 = tiles
        .iter()
        .enumerate()
        .filter(|&(i, (_, tile))| {
            (0..sides_count)
                .filter(|&side| {
                    tiles
                        .iter()
                        .enumerate()
                        .filter(|&(j, _)| j != i)
                        .any(|(_, (_, other_tile))| {
                            (0..transform_count).any(|transform| {
                                (0..tile_len).all(|i| {
                                    tile[to_index(apply_transforms([i, 0], side_transforms[side]))]
                                        == other_tile[to_index(apply_transforms([i, 0], transform))]
                                })
                            })
                        })
                })
                .take(3)
                .count()
                == 2
        })
        .map(|(_, &(id, _))| id as u64)
        .product();
    println!("{}", answer);
}
