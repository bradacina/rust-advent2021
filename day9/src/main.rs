use std::{fs::File, io::BufRead, io::BufReader, vec::Vec};

type Mat = Vec<Vec<u8>>;
struct Map {
    map: Mat,
    visits: Mat,
    num_rows: usize,
    num_cols: usize,
}

impl Map {
    fn item_at(&self, row: i16, col: i16) -> Option<u8> {
        if row < 0 {
            return None;
        }

        if col < 0 {
            return None;
        }

        if row >= self.num_rows as i16 {
            return None;
        }

        if col >= self.num_cols as i16 {
            return None;
        }

        Some(self.map[row as usize][col as usize])
    }

    fn visit_at(&self, row: i16, col: i16) -> Option<u8> {
        if row < 0 {
            return None;
        }

        if col < 0 {
            return None;
        }

        if row >= self.num_rows as i16 {
            return None;
        }

        if col >= self.num_cols as i16 {
            return None;
        }

        if self.visits[row as usize][col as usize] == 1 {
            return Some(1);
        }

        return None
    }
}

fn read_input(filename: &str) -> Map {
    let f = File::open(filename).expect(&("Cannot find file ".to_owned() + filename));
    let reader = BufReader::new(f);

    let mut result: Vec<Vec<u8>> = vec![];
    for line in reader.lines() {
        result.push(
            line.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect(),
        );
    }

    Map {
        num_rows: result.len(),
        num_cols: result[0].len(),
        map: result,
        visits: vec![],
    }
}

fn low_points(map: &Map) -> (Vec<u8>, Vec<(usize, usize)>) {
    let mut minimums: Vec<u8> = vec![];
    let mut positions: Vec<(usize, usize)> = vec![];

    for i in 0..map.num_rows {
        for j in 0..map.num_cols {
            let item = map.item_at(i as i16, j as i16).unwrap_or(99);
            let top = map.item_at(i as i16 - 1, j as i16).unwrap_or(99);
            let bottom = map.item_at((i + 1) as i16, j as i16).unwrap_or(99);
            let left = map.item_at(i as i16, j as i16 - 1).unwrap_or(99);
            let right = map.item_at(i as i16, (j + 1) as i16).unwrap_or(99);

            if top > item && bottom > item && left > item && right > item {
                minimums.push(item);

                positions.push((i, j));
            }
        }
    }

    (minimums, positions)
}

fn basins(map: &mut Map, lowest_positions: &Vec<(usize, usize)>) -> Vec<u16>{
    let mut basin_sizes: Vec<u16> = vec![];
    let mut visited: Vec<Vec<u8>> = vec![];

    for _ in 0..map.num_rows {
        visited.push(vec![0; map.num_cols]);
    }

    map.visits = visited;

    let directions: Vec<(i16, i16)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

    for (row, col) in lowest_positions {
        println!("inspecting basin at {},{}", row, col);
        map.visits[*row][*col] = 1;

        let mut basin_size = 1;
        let mut visit_stack: Vec<(usize, usize, i16)> = vec![];
        visit_stack.push((*row, *col, -1));

        loop {
            let last_visited = visit_stack.pop();
            if last_visited.is_none() {
                break;
            }

            let (last_row, last_col, last_direction) = last_visited.unwrap();

            let next_direction = last_direction + 1;
            if next_direction >= directions.len() as i16 {
                continue;
            }

            let next_row = last_row as i16 + directions[next_direction as usize].0;
            let next_col = last_col as i16 + directions[next_direction as usize].1;

            let next_item = map.item_at(next_row, next_col).unwrap_or(9);
            let next_visit = map.visit_at(next_row as i16, next_col as i16);
            if next_item == 9 || next_visit.is_some() {
                visit_stack.push((last_row, last_col, next_direction));
                continue;
            }

            visit_stack.push((last_row as usize, last_col as usize, next_direction));
            visit_stack.push((next_row as usize, next_col as usize, -1));
            map.visits[next_row as usize][next_col as usize] = 1;
            basin_size += 1;
        }

        println!("basin size: {}", basin_size);
        basin_sizes.push(basin_size);
    }

    basin_sizes
}

fn main() {
    let mut map = read_input("input.txt");
    let (lowest_points, lowest_positions) = low_points(&map);
    println!(
        "{:?}",
        lowest_points
            .iter()
            .fold(0 as u16, |a, item| a + (*item as u16) + 1)
    );

    let mut sizes = basins(&mut map, &lowest_positions);
    sizes.sort();
    sizes.reverse();
    println!("{:?}", sizes);

    println!("{}", sizes.iter().take(3).fold(1 as i32, |a, item| a * (*item as i32)));
}
