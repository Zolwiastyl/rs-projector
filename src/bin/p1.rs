fn get_input() -> &'static str {
    return r#"forward 5
down 5
forward 8
up 3
down 8
forward 2"#;
}

#[derive(Debug)]
struct Direction {
    x: i32,
    y: i32,
}
fn parse_line(line: &str) -> Direction {
    let (dir, amount) = line.split_once(" ").expect("Invalid input");
    let amount = amount.parse::<i32>().expect("Invalid input");
    return match dir {
        "forward" => Direction { x: amount, y: 0 },
        "up" => Direction { x: 0, y: -amount },
        "down" => Direction { x: 0, y: amount },
        _ => panic!("Invalid input"),
    };
}
fn main() {
    let input = get_input();
    let directions =
        input
            .lines()
            .map(parse_line)
            .fold(Direction { x: 0, y: 0 }, |acc, direction| Direction {
                x: acc.x + direction.x,
                y: acc.y + direction.y,
            });

    println!("result!");
    println!("x: {:?}", directions);
}
