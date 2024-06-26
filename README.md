# Maze Generator

A Rust-based command-line tool for generating and analyzing mazes using various algorithms.

## Features

- Generate mazes using three different algorithms:
  - Depth-First Search (DFS)
  - Prim's Algorithm
  - Kruskal's Algorithm
- Customize maze dimensions
- Analyze maze quality with metrics such as:
  - Number of dead ends
  - Longest path length
  - Average path length
  - Branching factor
- Calculate an overall quality index for generated mazes

## Installation

To use this maze generator, you need to have Rust and Cargo installed on your system. If you don't have them installed, you can get them from [rustup.rs](https://rustup.rs/).

1. Clone the repository:
   ```
   git clone https://github.com/yourusername/maze-generator.git
   cd maze-generator
   ```

2. Build the project:
   ```
   cargo build --release
   ```

The executable will be created in the `target/release` directory.

## Usage

To generate a maze, use the following command structure:

```
./target/release/mazegenerator -w <width> -h <height> -a <algorithm>
```

Where:
- `<width>` is the width of the maze
- `<height>` is the height of the maze
- `<algorithm>` is one of: `dfs`, `prim`, or `kruskal`

Example:
```
./target/release/mazegenerator -w 20 -g 20 -a dfs
```

This will generate a 20x20 maze using the Depth-First Search algorithm.

## Output

The program will output:
1. An ASCII representation of the generated maze
2. The time taken to generate the maze
3. Quality metrics for the maze:
   - Number of dead ends
   - Longest path length
   - Average path length
   - Branching factor
4. An overall quality index

## Contributing

Contributions to improve the maze generator are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- This project was inspired by various maze generation algorithms and their implementations.
- Thanks to the Rust community for providing excellent libraries and resources.