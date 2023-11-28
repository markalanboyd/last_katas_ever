use super::maze_generator::{Coordinate, Maze, MazePath};

pub fn maze_solver(maze: &Maze) -> MazePath {}

#[cfg(test)]
mod tests {
    use super::*;

    const MAZE_STR: &str = "
        ######
        S   ##
        # ## E
        #    #
        ######
    ";

    #[test]
    fn test_maze_solver() {
        let maze: Maze = Maze::from_str(MAZE_STR.trim());
        let solution: MazePath = vec![
            Coordinate { row: 1, col: 0 },
            Coordinate { row: 1, col: 1 },
            Coordinate { row: 2, col: 1 },
            Coordinate { row: 3, col: 1 },
            Coordinate { row: 3, col: 2 },
            Coordinate { row: 3, col: 3 },
            Coordinate { row: 3, col: 4 },
            Coordinate { row: 2, col: 4 },
            Coordinate { row: 2, col: 5 },
        ];
        assert_eq!(maze_solver(&maze), solution);
    }
}
