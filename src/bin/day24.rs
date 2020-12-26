use std::collections::{HashMap, HashSet};

fn main() {
    let input = "input/day24";
    let input = String::from_utf8(std::fs::read(input).unwrap()).unwrap();
    let mut black_tiles = HashSet::new();
    const DIR_TO_OFFSET: [(&str, [i32; 2]); 6] = [
        ("nw", [0, 1]),
        ("ne", [1, 1]),
        ("sw", [-1, -1]),
        ("se", [0, -1]),
        ("w", [-1, 0]),
        ("e", [1, 0]),
    ];
    const NEIGHBORS: [[i32; 2]; 6] = [[0, 1], [1, 1], [-1, -1], [0, -1], [-1, 0], [1, 0]];
    input
        .split_terminator('\n')
        .map(|mut line| {
            std::iter::from_fn(move || {
                if !line.is_empty() {
                    let (tail, dir) = DIR_TO_OFFSET
                        .iter()
                        .find_map(|&(prefix, dir)| {
                            line.strip_prefix(prefix).map(|tail| (tail, dir))
                        })
                        .unwrap();
                    line = tail;
                    Some(dir)
                } else {
                    None
                }
            })
            .fold([0, 0], |pos, dir| [pos[0] + dir[0], pos[1] + dir[1]])
        })
        .for_each(|flip_tile| {
            if !black_tiles.remove(&flip_tile) {
                black_tiles.insert(flip_tile);
            }
        });
    let answer = black_tiles.len();
    println!("{}", answer);

    // part2
    const CYCLE_COUNT: usize = 100;
    for _ in 0..CYCLE_COUNT {
        let remove_tiles: Vec<_> = black_tiles
            .iter()
            .copied()
            .filter(|&tile| {
                let neighbors_count = NEIGHBORS
                    .iter()
                    .map(|offset| [tile[0] + offset[0], tile[1] + offset[1]])
                    .filter(|neighbor| black_tiles.contains(neighbor))
                    .take(3)
                    .count();
                neighbors_count == 0 || neighbors_count > 2
            })
            .collect();
        let mut white_neighbors = HashMap::new();
        black_tiles
            .iter()
            .flat_map(|&tile| {
                NEIGHBORS
                    .iter()
                    .map(move |&offset| [tile[0] + offset[0], tile[1] + offset[1]])
                    .filter(|neighbor| !black_tiles.contains(neighbor))
            })
            .for_each(|white_neighbor| *white_neighbors.entry(white_neighbor).or_insert(0u8) += 1);

        remove_tiles.into_iter().for_each(|remove_tile| {
            let removed = black_tiles.remove(&remove_tile);
            assert!(removed);
        });
        white_neighbors
            .into_iter()
            .filter(|&(_, count)| count == 2)
            .for_each(|(tile, _)| {
                let inserted = black_tiles.insert(tile);
                assert!(inserted);
            });
    }
    let answer = black_tiles.len();
    println!("{}", answer);
}
