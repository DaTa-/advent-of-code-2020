fn main() {
    let input = std::fs::read("input/day11").unwrap();
    let width = input.iter().position(|&c| c == b'\n').unwrap();
    let height = input.len() / (width + 1);
    let mut state = input
        .into_iter()
        .filter(|c| !c.is_ascii_whitespace())
        .collect::<Vec<_>>();
    let neighbor_dirs = || {
        (-1isize..=1)
            .flat_map(|x| (-1..=1).map(move |y| [x, y]))
            .filter(|&[x, y]| x != 0 || y != 0)
    };
    let axis_iter = |i: usize, step: [isize; 2]| {
        (1isize..)
            .map(move |d| [d * step[0], d * step[1]])
            .scan((), move |(), d| {
                let [x, y] = [i % width, i / width];
                let [x, y] = [x as isize + d[0], y as isize + d[1]];
                if (0..width as isize).contains(&x) && (0..height as isize).contains(&y) {
                    Some([x as usize, y as usize])
                } else {
                    None
                }
            })
            .map(|[x, y]| y * width + x)
    };
    loop {
        let new_state = state
            .iter()
            .copied()
            .enumerate()
            .map(|(i, c)| {
                let viewed_neighbors = || {
                    neighbor_dirs().filter_map(|dir| {
                        axis_iter(i, dir)
                            .map(|j| state[j])
                            .filter(|&c| c != b'.')
                            .next()
                    })
                };
                match c {
                    b'L' => {
                        if viewed_neighbors().any(|c| c == b'#') {
                            b'L'
                        } else {
                            b'#'
                        }
                    }
                    b'#' => {
                        if viewed_neighbors().filter(|&c| c == b'#').count() >= 5 {
                            b'L'
                        } else {
                            b'#'
                        }
                    }
                    floor => floor,
                }
            })
            .collect();
        if new_state == state {
            break;
        }
        state = new_state;
    }
    let answer = state.into_iter().filter(|&c| c == b'#').count();
    println!("{}", answer);
}
