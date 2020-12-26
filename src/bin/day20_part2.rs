use std::collections::{HashMap, HashSet};

fn main() {
    let input = String::from_utf8(std::fs::read("input/day20").unwrap()).unwrap();
    let tile_len = input.split('\n').skip(1).next().unwrap().len();
    let mut tiles: Vec<(u16, Vec<bool>)> = input
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

    fn rotate([x, y]: [usize; 2]) -> [usize; 2] {
        [y, x]
    }
    fn flip(mut coords: [usize; 2], dim: usize, square_len: usize) -> [usize; 2] {
        coords[dim] = square_len - 1 - coords[dim];
        coords
    };
    const TRANSFORM_COUNT: usize = 1 << 3;
    const ROTATION: usize = 1 << 0;
    const FLIP0: usize = 1 << 1;
    const FLIP1: usize = 1 << 2;
    fn apply_transforms(mut coords: [usize; 2], transform: usize, square_len: usize) -> [usize; 2] {
        if transform & ROTATION != 0 {
            coords = rotate(coords);
        }
        if transform & FLIP0 != 0 {
            coords = flip(coords, 0, square_len);
        }
        if transform & FLIP1 != 0 {
            coords = flip(coords, 1, square_len);
        }
        coords
    }
    const SIDES_COUNT: usize = 4;
    let side_transforms = [0, FLIP1, ROTATION, ROTATION | FLIP0];
    let side_direction: [[isize; 2]; SIDES_COUNT] = [[0, -1], [0, 1], [-1, 0], [1, 0]];
    let opposite_side = [1, 0, 3, 2];
    fn to_index([x, y]: [usize; 2], width: usize) -> usize {
        x + width * y
    }
    fn to_coords(index: usize, width: usize) -> [usize; 2] {
        [index % width, index / width]
    }

    let mut mosaic = HashMap::<[isize; 2], (u16, Vec<bool>)>::new();
    mosaic.insert([0, 0], tiles.pop().unwrap());

    while let Some((position, index, transform)) = (0..tiles.len())
        .flat_map(|index| {
            mosaic
                .iter()
                .map(move |complete_tile| (complete_tile, index))
        })
        .flat_map(|(complete_tile, index)| {
            (0..TRANSFORM_COUNT).map(move |transform| (complete_tile, index, transform))
        })
        .flat_map(|(complete_tile, index, transform)| {
            (0..SIDES_COUNT).map(move |side| (complete_tile, side, index, transform))
        })
        .find_map(|(complete_tile, side, index, transform)| {
            let tile = &tiles[index].1;
            if (0..tile_len).map(|x| [x, 0]).all(|coords| {
                (complete_tile.1 .1)[to_index(
                    apply_transforms(coords, side_transforms[side], tile_len),
                    tile_len,
                )] == tile[to_index(
                    apply_transforms(
                        apply_transforms(coords, side_transforms[opposite_side[side]], tile_len),
                        transform,
                        tile_len,
                    ),
                    tile_len,
                )]
            }) {
                let dir = side_direction[side];
                let complete_coords = complete_tile.0;
                let position = [complete_coords[0] + dir[0], complete_coords[1] + dir[1]];
                Some((position, index, transform))
            } else {
                None
            }
        })
    {
        let tile = tiles.swap_remove(index);
        let transformed_tile: Vec<_> = (0..tile.1.len())
            .map(|i| {
                let coords = apply_transforms(to_coords(i, tile_len), transform, tile_len);
                tile.1[to_index(coords, tile_len)]
            })
            .collect();
        let previous = mosaic.insert(position, (tile.0, transformed_tile));
        assert!(previous.is_none());
    }
    assert_eq!(tiles.len(), 0);

    let mosaic_span = [
        [
            mosaic.keys().map(|&[x, _]| x).min().unwrap(),
            mosaic.keys().map(|&[x, _]| x).max().unwrap(),
        ],
        [
            mosaic.keys().map(|&[_, y]| y).min().unwrap(),
            mosaic.keys().map(|&[_, y]| y).max().unwrap(),
        ],
    ];

    let image_len = (tile_len - 2) * (mosaic_span[0][1] - mosaic_span[0][0] + 1) as usize;
    let image: Vec<_> = (mosaic_span[1][0]..=mosaic_span[1][1])
        .flat_map(|y| {
            (0..tile_len)
                .filter(|&row| row != 0 && row != tile_len - 1)
                .map(move |row| (y, row))
        })
        .flat_map(|(y, row)| {
            (mosaic_span[0][0]..=mosaic_span[0][1])
                .map(move |x| ([x, y], row))
                .flat_map(move |(mosaic_coords, row)| {
                    (0..tile_len)
                        .filter(move |&x| x != 0 && x != tile_len - 1)
                        .map(move |x| (mosaic_coords, [x, row]))
                })
        })
        .map(|(mosaic_coords, coords)| (mosaic[&mosaic_coords].1)[to_index(coords, tile_len)])
        .collect();
    assert_eq!(image.len(), image_len * image_len);

    let monster = String::from_utf8(std::fs::read("input/day20_2").unwrap()).unwrap();
    let monster_width = monster.split_terminator('\n').next().unwrap().len();
    let monster_sizes = [monster_width, monster.len() / (monster_width + 1)];
    let monster_occupied: Vec<[usize; 2]> = monster
        .split_terminator('\n')
        .flat_map(|line| line.chars())
        .enumerate()
        .filter(|&(_, ch)| ch == '#')
        .map(|(i, _)| to_coords(i, monster_sizes[0]))
        .collect();

    let most_suitable_transform = (0..TRANSFORM_COUNT)
        .max_by_key(|&transform| {
            (0..image_len - monster_sizes[0] + 1)
                .flat_map(|x| (0..image_len - monster_sizes[1] + 1).map(move |y| [x, y]))
                .filter(|[x, y]| {
                    monster_occupied
                        .iter()
                        .map(|&[mx, my]| [x + mx, y + my])
                        .all(|coords| {
                            image[to_index(
                                apply_transforms(coords, transform, image_len),
                                image_len,
                            )]
                        })
                })
                .count()
        })
        .unwrap();

    let excluded_coords: HashSet<_> = (0..image_len - monster_sizes[0] + 1)
        .flat_map(|x| (0..image_len - monster_sizes[1] + 1).map(move |y| [x, y]))
        .filter(|[x, y]| {
            monster_occupied
                .iter()
                .map(|&[mx, my]| [x + mx, y + my])
                .all(|coords| {
                    image[to_index(
                        apply_transforms(coords, most_suitable_transform, image_len),
                        image_len,
                    )]
                })
        })
        .flat_map(|[x, y]| {
            monster_occupied
                .iter()
                .map(move |&[mx, my]| [x + mx, y + my])
        })
        .collect();
    let answer = image.iter().filter(|&&c| c).count() - excluded_coords.len();
    println!("{}", answer);
}
