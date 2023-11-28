use super::maze_generator::{CellType, Coordinate, Maze, MazePath};

const DIR: [[isize; 2]; 4] = [[-1, 0], [1, 0], [0, -1], [0, 1]];

pub fn maze_solver(maze: &Maze) -> MazePath {
    let mut maze_path: MazePath = MazePath::new();
    fn walk(maze: &Maze, curr: Coordinate, maze_path: &mut MazePath) -> bool {
        match maze.type_of(curr) {
            CellType::End => {
                maze_path.push(curr);
                return true;
            }
            _ => false,
        }

        maze_path.push(curr);

        for i in 0..DIR.len() {
            let [x, y] = DIR[i];
            if walk(&maze, curr, maze_path) {
                true
            }
        }
    }
    let mut current: Option<Coordinate> = maze.find_start();
    walk(&maze, current, maze_path);
    maze_path
}

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
