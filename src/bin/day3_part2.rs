fn main() {
    let input = std::fs::read("input/day3").unwrap();
    let width = input.iter().position(|&c| c == b'\n').unwrap();
    let height = input.len() / (width + 1);
    let cell_occupied = |x, y| input[y * (width + 1) + x] == b'#';
    const SLOPES: &[[usize; 2]] = &[[1, 1], [3, 1], [5, 1], [7, 1], [1, 2]];
    let answer = SLOPES
        .iter()
        .map(|slope| {
            (1..((height - 1) / slope[1]) + 1)
                .map(|y| [(0 + slope[0] * y) % width, 0 + y * slope[1]])
                .filter(|&[x, y]| cell_occupied(x, y))
                .count()
        })
        .product::<usize>();
    println!("{}", answer);
}
