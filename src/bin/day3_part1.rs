fn main() {
    let input = std::fs::read("input/day3").unwrap();
    let width = input.iter().position(|&c| c == b'\n').unwrap();
    let height = input.len() / (width + 1);
    let cell_occupied = |x, y| input[y * (width + 1) + x] == b'#';
    const SLOPE: [usize; 2] = [3, 1];
    let tree_count = (1..height)
        .map(|y| [(0 + SLOPE[0] * y) % width, 0 + y * SLOPE[1]])
        .filter(|&[x, y]| cell_occupied(x, y))
        .count();
    println!("{}", tree_count);
}
