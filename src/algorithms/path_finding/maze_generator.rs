use std::cell::Cell;

#[derive(Clone, Copy, Debug, PartialEq)]
enum CellType {
    Wall,
    Path,
    Start,
    End,
}

pub struct Maze {
    grid: Vec<Vec<CellType>>,
}

impl Maze {
    pub fn from_str(maze_str: &str) -> Self {
        let grid = maze_str
            .lines()
            .map(|line| {
                line.trim_start()
                    .chars()
                    .map(|ch| match ch {
                        '#' => CellType::Wall,
                        ' ' => CellType::Path,
                        'S' => CellType::Start,
                        'E' => CellType::End,
                        _ => panic!("Invalid character in maze definition"),
                    })
                    .collect()
            })
            .collect();

        Maze { grid }
    }

    fn find_cell_type(&self, cell_type: CellType) -> Option<Coordinate> {
        for (row_index, row) in self.grid.iter().enumerate() {
            for (col_index, &cell) in row.iter().enumerate() {
                if cell == cell_type {
                    return Some(Coordinate {
                        row: row_index,
                        col: col_index,
                    });
                }
            }
        }
        None
    }

    pub fn find_start(&self) -> Option<Coordinate> {
        self.find_cell_type(CellType::Start)
    }

    pub fn find_end(&self) -> Option<Coordinate> {
        self.find_cell_type(CellType::End)
    }
}

#[derive(Debug, PartialEq)]
pub struct Coordinate {
    pub row: usize,
    pub col: usize,
}

pub type MazePath = Vec<Coordinate>;

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
    fn test_find_start() {
        let maze: Maze = Maze::from_str(MAZE_STR.trim());
        let expected: Option<Coordinate> = Some(Coordinate { row: 1, col: 0 });
        assert_eq!(maze.find_start(), expected);
    }

    #[test]
    fn test_find_end() {
        let maze: Maze = Maze::from_str(MAZE_STR.trim());
        let expected: Option<Coordinate> = Some(Coordinate { row: 2, col: 5 });
        assert_eq!(maze.find_end(), expected);
    }
}
