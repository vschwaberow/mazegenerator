use clap::{value_parser, Arg, Command};
use rand::prelude::*;
use std::time::Instant;

struct Cell {
    x: usize,
    y: usize,
    visited: bool,
    walls: [bool; 4],
}

struct Maze {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

struct MazeQuality {
    dead_ends: usize,
    longest_path: usize,
    avg_path_length: f64,
    branching_factor: f64,
}

impl Maze {
    fn new(width: usize, height: usize) -> Self {
        let cells = (0..height)
            .flat_map(|y| {
                (0..width).map(move |x| Cell {
                    x,
                    y,
                    visited: false,
                    walls: [true, true, true, true],
                })
            })
            .collect();

        Maze {
            width,
            height,
            cells,
        }
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn remove_wall(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        let idx1 = self.get_index(x1, y1);
        let idx2 = self.get_index(x2, y2);

        if y1 < y2 {
            self.cells[idx1].walls[2] = false;
            self.cells[idx2].walls[0] = false;
        } else if y1 > y2 {
            self.cells[idx1].walls[0] = false;
            self.cells[idx2].walls[2] = false;
        } else if x1 < x2 {
            self.cells[idx1].walls[1] = false;
            self.cells[idx2].walls[3] = false;
        } else {
            self.cells[idx1].walls[3] = false;
            self.cells[idx2].walls[1] = false;
        }
    }
    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = self.get_index(x, y);
                print!(
                    "{}{}{}",
                    if x == 0 { "+" } else { "" },
                    if self.cells[idx].walls[0] {
                        "---"
                    } else {
                        "   "
                    },
                    "+"
                );
            }
            println!();

            for x in 0..self.width {
                let idx = self.get_index(x, y);
                print!(
                    "{}{}",
                    if self.cells[idx].walls[3] { "|" } else { " " },
                    "   "
                );
            }
            println!("|");
        }

        for _x in 0..self.width {
            print!("+---");
        }
        println!("+");
    }

    fn measure_quality(&self) -> MazeQuality {
        let dead_ends = self.count_dead_ends();
        let (longest_path, total_path_length, total_paths) = self.measure_paths();
        let branching_factor = self.calculate_branching_factor();

        MazeQuality {
            dead_ends,
            longest_path,
            avg_path_length: total_path_length as f64 / total_paths as f64,
            branching_factor,
        }
    }

    fn count_dead_ends(&self) -> usize {
        self.cells
            .iter()
            .filter(|&cell| cell.walls.iter().filter(|&&wall| wall).count() == 3)
            .count()
    }

    fn measure_paths(&self) -> (usize, usize, usize) {
        let mut longest_path = 0;
        let mut total_path_length = 0;
        let mut total_paths = 0;

        for start_cell in &self.cells {
            let (path_length, path_count) = {
                let start_x = start_cell.x;
                let start_y = start_cell.y;
                self.longest_path_from(start_x, start_y)
            };
            longest_path = longest_path.max(path_length);
            total_path_length += path_length;
            total_paths += path_count;
        }

        (longest_path, total_path_length, total_paths)
    }

    fn longest_path_from(&self, start_x: usize, start_y: usize) -> (usize, usize) {
        let mut visited = vec![vec![false; self.width]; self.height];
        self.dfs_longest_path(start_x, start_y, &mut visited, 0)
    }

    fn dfs_longest_path(
        &self,
        x: usize,
        y: usize,
        visited: &mut Vec<Vec<bool>>,
        length: usize,
    ) -> (usize, usize) {
        visited[y][x] = true;
        let mut max_length = length;
        let mut path_count = 0;

        let directions = [(0, -1), (1, 0), (0, 1), (-1, 0)];
        for (i, &(dx, dy)) in directions.iter().enumerate() {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0 && nx < self.width as i32 && ny >= 0 && ny < self.height as i32 {
                let nx = nx as usize;
                let ny = ny as usize;
                if !self.cells[self.get_index(x, y)].walls[i] && !visited[ny][nx] {
                    let (sub_length, sub_count) =
                        self.dfs_longest_path(nx, ny, visited, length + 1);
                    max_length = max_length.max(sub_length);
                    path_count += sub_count;
                }
            }
        }

        visited[y][x] = false;
        (max_length, if path_count == 0 { 1 } else { path_count })
    }

    fn calculate_branching_factor(&self) -> f64 {
        let total_branches: usize = self
            .cells
            .iter()
            .map(|cell| 4 - cell.walls.iter().filter(|&&wall| wall).count())
            .sum();

        total_branches as f64 / (self.width * self.height) as f64
    }
}

fn kruskal(maze: &mut Maze) {
    let mut rng = thread_rng();
    let mut sets: Vec<usize> = (0..maze.width * maze.height).collect();
    let mut walls: Vec<(usize, usize, usize, usize)> = Vec::new();

    for y in 0..maze.height {
        for x in 0..maze.width {
            if x < maze.width - 1 {
                walls.push((x, y, x + 1, y));
            }
            if y < maze.height - 1 {
                walls.push((x, y, x, y + 1));
            }
        }
    }

    walls.shuffle(&mut rng);

    for (x1, y1, x2, y2) in walls {
        let idx1 = maze.get_index(x1, y1);
        let idx2 = maze.get_index(x2, y2);

        let set1 = find(&mut sets, idx1);
        let set2 = find(&mut sets, idx2);

        if set1 != set2 {
            maze.remove_wall(x1, y1, x2, y2);
            union(&mut sets, set1, set2);
        }
    }
}

fn find(sets: &mut Vec<usize>, x: usize) -> usize {
    if sets[x] != x {
        sets[x] = find(sets, sets[x]);
    }
    sets[x]
}

fn union(sets: &mut Vec<usize>, x: usize, y: usize) {
    let root_x = find(sets, x);
    let root_y = find(sets, y);
    sets[root_x] = root_y;
}

fn prim(maze: &mut Maze) {
    let mut rng = thread_rng();
    let start_x = rng.gen_range(0..maze.width);
    let start_y = rng.gen_range(0..maze.height);
    let mut frontier = vec![(start_x, start_y)];
    let maze_index = maze.get_index(start_x, start_y);
    maze.cells[maze_index].visited = true;

    while !frontier.is_empty() {
        let idx = rng.gen_range(0..frontier.len());
        let (x, y) = frontier.swap_remove(idx);

        let neighbors = [
            (x, y.wrapping_sub(1)),
            (x + 1, y),
            (x, y + 1),
            (x.wrapping_sub(1), y),
        ];

        for &(nx, ny) in &neighbors {
            if nx < maze.width && ny < maze.height {
                let n_idx = maze.get_index(nx, ny);
                let is_unvisited = !maze.cells[n_idx].visited;
                if is_unvisited {
                    maze.remove_wall(x, y, nx, ny);
                    maze.cells[n_idx].visited = true;
                    frontier.push((nx, ny));
                }
            }
        }
    }
}

fn dfs(maze: &mut Maze) {
    let mut rng = thread_rng();
    let mut stack = vec![(0, 0)];
    maze.cells[0].visited = true;

    while let Some(&(x, y)) = stack.last() {
        let mut neighbors = Vec::new();
        let directions = [(0, -1), (1, 0), (0, 1), (-1, 0)];

        for (dx, dy) in directions.iter() {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0 && nx < maze.width as i32 && ny >= 0 && ny < maze.height as i32 {
                let n_idx = maze.get_index(nx as usize, ny as usize);
                if !maze.cells[n_idx].visited {
                    neighbors.push((nx as usize, ny as usize));
                }
            }
        }

        if !neighbors.is_empty() {
            let &(nx, ny) = neighbors.choose(&mut rng).unwrap();
            maze.remove_wall(x, y, nx, ny);
            let maze_index = maze.get_index(nx, ny);
            maze.cells[maze_index].visited = true;
            stack.push((nx, ny));
        } else {
            stack.pop();
        }
    }
}

fn calculate_quality_index(quality: &MazeQuality, maze_size: usize) -> f64 {
    let dead_end_ratio = quality.dead_ends as f64 / maze_size as f64;
    let path_length_ratio = quality.longest_path as f64 / maze_size as f64;
    let normalized_avg_path = quality.avg_path_length / maze_size as f64;

    let w_dead_ends = 0.25;
    let w_longest_path = 0.3;
    let w_avg_path = 0.25;
    let w_branching = 0.2;

    (1.0 - dead_end_ratio) * w_dead_ends
        + path_length_ratio * w_longest_path
        + normalized_avg_path * w_avg_path
        + quality.branching_factor * w_branching
}

fn main() {
    let matches = Command::new("Maze Generator")
        .version("1.0")
        .author("Volker Schwaberow <volker@schwaberow.de>")
        .about("Generates mazes using various algorithms")
        .arg(
            Arg::new("width")
                .short('w')
                .long("width")
                .value_name("WIDTH")
                .help("Sets the width of the maze")
                .required(true)
                .value_parser(value_parser!(usize)),
        )
        .arg(
            Arg::new("height")
                .short('g')
                .long("height")
                .value_name("HEIGHT")
                .help("Sets the height of the maze")
                .required(true)
                .value_parser(value_parser!(usize)),
        )
        .arg(
            Arg::new("algorithm")
                .short('a')
                .long("algorithm")
                .value_name("ALGORITHM")
                .help("Sets the algorithm to use (kruskal, prim, or dfs)")
                .required(true)
                .value_parser(["kruskal", "prim", "dfs"]),
        )
        .get_matches();

    let width = *matches.get_one::<usize>("width").unwrap();
    let height = *matches.get_one::<usize>("height").unwrap();
    let algorithm = matches.get_one::<String>("algorithm").unwrap();

    let mut maze = Maze::new(width, height);

    let start = Instant::now();

    match algorithm.as_str() {
        "kruskal" => kruskal(&mut maze),
        "prim" => prim(&mut maze),
        "dfs" => dfs(&mut maze),
        _ => unreachable!(),
    }

    let duration = start.elapsed();

    println!("Maze generated using {} algorithm:", algorithm);
    maze.print();
    println!("Time taken: {:?}", duration);

    let quality = maze.measure_quality();
    let quality_index = calculate_quality_index(&quality, width * height);

    println!("\nMaze Quality Metrics:");
    println!("Dead ends: {}", quality.dead_ends);
    println!("Longest path: {}", quality.longest_path);
    println!("Average path length: {:.2}", quality.avg_path_length);
    println!("Branching factor: {:.2}", quality.branching_factor);
    println!("Quality Index: {:.4}", quality_index);
}
