const INPUT: &str = "7777838353
2217272478
3355318645
2242618113
7182468666
5441641111
4773862364
5717125521
7542127721
4576678341";

// const INPUT: &str = "5483143223
// 2745854711
// 5264556173
// 6141336146
// 6357385478
// 4167524645
// 2176841721
// 6882881134
// 4846848554
// 5283751526";

type Map = [[i8; 10]; 10];

fn print_map(message: &str, map: &Map) {
    println!("{}", message);
    for row in map {
        println!("{:>3?}", row);
    }
    println!("");
}

fn parse_input() -> Map {
    let x = INPUT
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    let mut result: Map = [[0; 10]; 10];

    for i in 0..10 {
        for j in 0..10 {
            result[i][j] = x[i][j].to_digit(10).unwrap() as i8;
        }
    }

    result
}

fn add_neightbour_flash(
    coords: Option<(usize, usize)>,
    map: &mut Map,
    new_flash_positions: &mut Vec<(usize, usize)>,
) {
    if coords.is_none() {
        return;
    }

    let (i, j) = coords.unwrap();
    if map[i][j] != -1 {
        // prevent neighbours from flashing eachother recursively
        // println!("incrementing {},{} due to nearby flash", i,j);
        map[i][j] += 1;
    }

    if map[i][j] > 9 {
        // println!("adding new flash position {}, {}", i, j);
        new_flash_positions.push((i, j));
    }
}

fn clamp_coords(i: i8, j: i8) -> Option<(usize, usize)> {
    if i < 0 {
        return None;
    }
    if i >= 10 {
        return None;
    }
    if j < 0 {
        return None;
    }
    if j >= 10 {
        return None;
    }

    return Some((i as usize, j as usize));
}

fn flash(map: &mut Map, flash_positions: &Vec<(usize, usize)>) -> (Vec<(usize, usize)>, usize) {
    let mut result: Vec<(usize, usize)> = vec![];
    let mut num_flashes = 0;
    for &(i, j) in flash_positions {
        if map[i][j] == -1 {
            continue;
        }
        num_flashes += 1;
        map[i][j] = -1;

        // println!("executing flash at {},{}", i,j);

        add_neightbour_flash(clamp_coords(i as i8 - 1, j as i8 - 1), map, &mut result);
        add_neightbour_flash(clamp_coords(i as i8 - 1, j as i8), map, &mut result);
        add_neightbour_flash(clamp_coords(i as i8 - 1, j as i8 + 1), map, &mut result);
        add_neightbour_flash(clamp_coords(i as i8, j as i8 - 1), map, &mut result);
        add_neightbour_flash(clamp_coords(i as i8, j as i8 + 1), map, &mut result);
        add_neightbour_flash(clamp_coords(i as i8 + 1, j as i8 - 1), map, &mut result);
        add_neightbour_flash(clamp_coords(i as i8 + 1, j as i8), map, &mut result);
        add_neightbour_flash(clamp_coords(i as i8 + 1, j as i8 + 1), map, &mut result);
    }

    result.sort_unstable();
    result.dedup();
    (result, num_flashes)
}

fn part1(map: &mut Map) {
    let mut total_flashes = 0;
    for step in 0..2000 {
        let mut total_step_flashes = 0;
        let mut flash_positions: Vec<(usize, usize)> = vec![];

        for i in 0..10 {
            for j in 0..10 {
                map[i][j] += 1;
                if map[i][j] > 9 {
                    flash_positions.push((i, j));
                }
            }
        }

        // print_map("after additioon", map);

        loop {
            // println!("flashes {:?}", flash_positions);

            let flash_ret = flash(map, &flash_positions);
            flash_positions = flash_ret.0;
            total_flashes += flash_ret.1;
            total_step_flashes += flash_ret.1;

            // print_map("after flash", map);

            if flash_positions.len() == 0 {
                break;
            }
        }

        // println!("{} flashes on step {}", total_step_flashes, step);

        for i in 0..10 {
            for j in 0..10 {
                if map[i][j] == -1 {
                    map[i][j] = 0;
                }
            }
        }

        // print_map("end of step", map);
        if total_step_flashes == 100 {
            println!("all octopuses flash on step {}", step+1);
            return;
        }
    }

    println!("{}", total_flashes);
}

fn main() {
    let mut map = parse_input();
    part1(&mut map);
}
