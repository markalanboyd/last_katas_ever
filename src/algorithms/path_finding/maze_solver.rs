// TODO Refactor this to have a generator to make it more flexible.

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Point {
    x: usize,
    y: usize,
}

#[derive(PartialEq, Debug)]
pub struct Path {
    points: Vec<Point>,
}

impl Path {
    fn new() -> Path {
        Path { points: Vec::new() }
    }

    fn push(&mut self, point: Point) {
        self.points.push(point);
    }

    fn pop(&mut self) -> Option<Point> {
        self.points.pop()
    }
}

const DIR: [[isize; 2]; 4] = [[-1, 0], [1, 0], [0, -1], [0, 1]];

fn walk(
    maze: &Vec<Vec<char>>,
    wall: char,
    curr: Point,
    end: Point,
    seen: &mut Vec<Vec<bool>>,
    maze_path: &mut Path,
) -> bool {
    if curr.x >= maze[0].len() || curr.y >= maze.len() {
        return false;
    }
    if maze[curr.y][curr.x] == wall {
        return false;
    }
    if seen[curr.y][curr.x] {
        return false;
    }
    if curr.x == end.x && curr.y == end.y {
        maze_path.push(end);
        return true;
    }

    seen[curr.y][curr.x] = true;
    maze_path.push(curr);

    for &[dx, dy] in DIR.iter() {
        let new_x: isize = curr.x as isize + dx;
        let new_y: isize = curr.y as isize + dy;
        if new_x >= 0 && new_y >= 0 {
            let new_curr: Point = Point {
                x: new_x as usize,
                y: new_y as usize,
            };
            if walk(maze, wall, new_curr, end, seen, maze_path) {
                return true;
            }
        }
    }

    maze_path.pop();

    return false;
}

pub fn solve_maze(maze: &Vec<Vec<char>>, wall: char, start: Point, end: Point) -> Path {
    let mut maze_path: Path = Path::new();
    let mut seen: Vec<Vec<bool>> = vec![vec![false; maze[0].len()]; maze.len()];

    walk(&maze, wall, start, end, &mut seen, &mut maze_path);

    maze_path
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_maze() {
        let maze: Vec<Vec<char>> = vec![
            vec!['#', '#', '#', '#', '#', 'E', '#'],
            vec!['#', ' ', ' ', ' ', ' ', ' ', '#'],
            vec!['#', 'S', '#', '#', '#', '#', '#'],
        ];

        let wall: char = '#';
        let start: Point = Point { x: 1, y: 2 };
        let end: Point = Point { x: 5, y: 0 };
        let result: Path = solve_maze(&maze, wall, start, end);
        let expected: Path = Path {
            points: vec![
                Point { x: 1, y: 2 },
                Point { x: 1, y: 1 },
                Point { x: 2, y: 1 },
                Point { x: 3, y: 1 },
                Point { x: 4, y: 1 },
                Point { x: 5, y: 1 },
                Point { x: 5, y: 0 },
            ],
        };
        assert_eq!(result, expected);
    }
}
