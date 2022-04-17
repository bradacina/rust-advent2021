use std::io::BufRead;

#[derive(Debug)]
struct Cucumber {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Map {
    map: Vec<Vec<bool>>,
    width: usize,
    height: usize,
    east_flock: Vec<Cucumber>,
    south_flock: Vec<Cucumber>,
}

impl Map {
    fn fill_map(&mut self) {
        self.map = vec![];
        for _ in 0..self.height {
            self.map.push(vec![false; self.width]);
        }

        for cucumber in self.east_flock.iter() {
            self.map[cucumber.x][cucumber.y] = true;
        }

        for cucumber in self.south_flock.iter() {
            self.map[cucumber.x][cucumber.y] = true;
        }
    }

    fn clamp_coords(&self, i: usize, j: usize) -> (usize, usize) {
        let mut clamped_i = i;
        let mut clamped_j = j;
        if clamped_i >= self.height {
            clamped_i = clamped_i - self.height;
        }

        if clamped_j >= self.width {
            clamped_j = clamped_j - self.width;
        }

        (clamped_i, clamped_j)
    }

    fn do_move(&mut self) -> usize{
        let east_moves = self.east_move();

        self.fill_map();

        let south_moves = self.south_move();

        self.fill_map();

        east_moves + south_moves
    }

    fn east_move(&mut self) -> usize {
        let mut num_moves: usize = 0;
        for i in 0..self.east_flock.len() {
            let (new_x, new_y) = self.clamp_coords(self.east_flock[i].x, self.east_flock[i].y + 1);

            if !self.map[new_x][new_y] {
                self.east_flock[i].x = new_x;
                self.east_flock[i].y = new_y;
                num_moves += 1;
            }
        }

        num_moves
    }

    fn south_move(&mut self) -> usize {
        let mut num_moves: usize = 0;
        for i in 0..self.south_flock.len() {
            let (new_x, new_y) =
                self.clamp_coords(self.south_flock[i].x + 1, self.south_flock[i].y);

            if !self.map[new_x][new_y] {
                num_moves += 1;
                self.south_flock[i].x = new_x;
                self.south_flock[i].y = new_y;
            }
        }

        num_moves
    }
}

fn read_input(filename: &str) -> Map {
    let f = std::fs::File::open(filename).expect("cannot open file");

    let reader = std::io::BufReader::new(f);

    let mut map = Map {
        map: vec![],
        width: 0,
        height: 0,
        east_flock: vec![],
        south_flock: vec![],
    };

    let mut max_width = 0;

    for (row_index, line) in reader.lines().enumerate() {
        map.height += 1;
        for (col_index, ch) in line.unwrap().chars().enumerate() {
            if col_index > max_width {
                max_width = col_index;
            }

            match ch {
                '>' => map.east_flock.push(Cucumber {
                    x: row_index,
                    y: col_index,
                }),
                'v' => map.south_flock.push(Cucumber {
                    x: row_index,
                    y: col_index,
                }),
                _ => (),
            }
        }
    }

    map.width = max_width + 1;

    map.fill_map();

    map
}

fn part1(map: &mut Map) {
    for step in 0..1000 {
        let moves = map.do_move();

        if moves == 0 {
            println!("no moves at step {}", step+1);
            break;
        }
    }
}
fn main() {
    let mut map = read_input("input.txt");
    part1(&mut map);
}
