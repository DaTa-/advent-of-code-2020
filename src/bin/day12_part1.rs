fn main() {
    let mut dir = [1, 0];
    let mut coords = [0, 0];
    String::from_utf8(std::fs::read("input/day12").unwrap())
        .unwrap()
        .split_terminator("\n")
        .map(|n| (n.chars().next().unwrap(), n[1..].parse::<i32>().unwrap()))
        .for_each(|(key, value)| match key {
            'N' => coords[1] += value,
            'S' => coords[1] -= value,
            'E' => coords[0] += value,
            'W' => coords[0] -= value,
            'F' => coords = [coords[0] + value * dir[0], coords[1] + value * dir[1]],
            key => {
                let value = if key == 'L' { 1 } else { 3 } * (value / 90) % 4;
                let mut rotate_left = || dir = [-dir[1], dir[0]];
                for _ in 0..value {
                    rotate_left();
                }
            }
        });
    let answer = coords[0].abs() + coords[1].abs();
    println!("{}", answer);
}
