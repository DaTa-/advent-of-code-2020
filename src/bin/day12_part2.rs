fn main() {
    let mut coords = [10, 1];
    let mut ship_coords = [0, 0];
    String::from_utf8(std::fs::read("input/day12").unwrap())
        .unwrap()
        .split_terminator("\n")
        .map(|n| (n.chars().next().unwrap(), n[1..].parse::<i32>().unwrap()))
        .for_each(|(key, value)| match key {
            'N' => coords[1] += value,
            'S' => coords[1] -= value,
            'E' => coords[0] += value,
            'W' => coords[0] -= value,
            'F' => {
                ship_coords = [
                    ship_coords[0] + value * coords[0],
                    ship_coords[1] + value * coords[1],
                ]
            }
            key => {
                let value = if key == 'L' { 1 } else { 3 } * (value / 90) % 4;
                let mut rotate_left = || coords = [-coords[1], coords[0]];
                for _ in 0..value {
                    rotate_left();
                }
            }
        });
    let answer = ship_coords[0].abs() + ship_coords[1].abs();
    println!("{}", answer);
}
