fn main() {
    let input = std::fs::read("input/day11").unwrap();
    let width = input.iter().position(|&c| c == b'\n').unwrap();
    let height = input.len() / (width + 1);
    let mut state = input
        .into_iter()
        .filter(|c| !c.is_ascii_whitespace())
        .collect::<Vec<_>>();
    let adjacent = |i: usize| {
        (-1..=1)
            .flat_map(|x| (-1isize..=1).map(move |y| [x, y]))
            .filter(|&[x, y]| x != 0 || y != 0)
            .filter_map(move |[dx, dy]| {
                let [x, y] = [i % width, i / width];
                let [x, y] = [x as isize + dx, y as isize + dy];
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
            .map(|(i, c)| match c {
                b'L' => {
                    if adjacent(i).map(|j| state[j]).any(|c| c == b'#') {
                        b'L'
                    } else {
                        b'#'
                    }
                }
                b'#' => {
                    if adjacent(i).map(|j| state[j]).filter(|&c| c == b'#').count() >= 4 {
                        b'L'
                    } else {
                        b'#'
                    }
                }
                floor => floor,
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
