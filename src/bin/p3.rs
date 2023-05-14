fn get_input() -> &'static str {
    return "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
}

fn main() {
    let trees_count = get_input()
        .lines()
        .enumerate()
        .flat_map(|(idx, row)| row.chars().nth(idx * 3 % row.len()))
        .filter(|char_at| char_at == &'#')
        .count();

    println!("{}", trees_count);
    // filter(|&(idx, line)| {
    //     let char_at = line.chars().nth(idx * 3 % line.len());

    //     if let Some(char_at) = char_at {
    //         if (char_at == '#') {
    //             return true;
    //         }
    //         return false;
    //     }

    //     return false;
    // });
}
