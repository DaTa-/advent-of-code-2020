use std::collections::{HashMap, HashSet};

const PART2: bool = true;
const DIM_COUNT: usize = if PART2 { 4 } else { 3 };
const CYCLES: usize = 6;
type CoordType = i8;

fn main() {
    let input = String::from_utf8(std::fs::read("input/day17").unwrap()).unwrap();
    let mut active_cubes: HashSet<[CoordType; DIM_COUNT]> = input
        .split_terminator('\n')
        .enumerate()
        .flat_map(|(i, r)| r.chars().enumerate().map(move |(j, c)| ([i, j], c)))
        .filter_map(|(coords, c)| {
            let mut result = [0; DIM_COUNT];
            result
                .iter_mut()
                .zip(coords.iter())
                .for_each(|(r, &c)| *r = c as _);
            Some(result).filter(|_| c == '#')
        })
        .collect();
    const ADJACENT_SPAN: CoordType = 3;
    let adjacent_offsets: Vec<_> = (0..(ADJACENT_SPAN.pow(DIM_COUNT as _)))
        .filter(|&i| i != ADJACENT_SPAN.pow(DIM_COUNT as _) / 2)
        .map(|i| {
            let mut result = [0; DIM_COUNT];
            result.iter_mut().enumerate().for_each(|(dim, coord)| {
                *coord = i % ADJACENT_SPAN.pow((dim + 1) as _) / ADJACENT_SPAN.pow(dim as _) - 1
            });
            result
        })
        .collect();
    let add = |a: &[CoordType; DIM_COUNT], b: &[CoordType; DIM_COUNT]| -> [CoordType; DIM_COUNT] {
        let mut c = [0; DIM_COUNT];
        c.iter_mut()
            .zip(a.iter().zip(b.iter()))
            .for_each(|(c, (&a, &b))| *c = a + b);
        c
    };
    let mut deactivate = Vec::new();
    let mut inactive_neighbors = HashMap::<_, usize>::new();
    for _ in 0..CYCLES {
        for cube in active_cubes.iter() {
            let active_neighbors_count = adjacent_offsets
                .iter()
                .map(|o| add(cube, o))
                .filter(|neighbor| active_cubes.contains(neighbor))
                .take(4)
                .count();
            if active_neighbors_count != 2 && active_neighbors_count != 3 {
                deactivate.push(cube.clone());
            }
            adjacent_offsets
                .iter()
                .map(|o| add(cube, o))
                .filter(|neighbor| !active_cubes.contains(neighbor))
                .for_each(|inactive_neighbor| {
                    *inactive_neighbors.entry(inactive_neighbor).or_insert(0) += 1
                });
        }
        deactivate.drain(..).for_each(|deactivate| {
            active_cubes.remove(&deactivate);
        });
        inactive_neighbors
            .drain()
            .filter_map(|n| Some(n.0).filter(|_| n.1 == 3))
            .for_each(|activate| {
                active_cubes.insert(activate);
            });
    }
    let answer = active_cubes.len();
    println!("{:?}", answer);
}
