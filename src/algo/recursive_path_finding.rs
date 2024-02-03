// TODO: add documentations

#[derive(Debug, Clone, PartialEq, Eq)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Point {
        Point { x, y }
    }
}

impl From<(isize, isize)> for Point {
    fn from((x, y): (isize, isize)) -> Self {
        Point { x, y }
    }
}

const DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn walk(
    maze: &Vec<Vec<&str>>,
    wall: &str,
    curr: Point,
    end: Point,
    already_walked: &mut Vec<Vec<bool>>,
    path: &mut Vec<Point>,
) -> bool {
    // Base Cases
    // 1. Off the map
    if curr.x < 0 || curr.x >= maze[0].len() as isize || curr.y < 0 || curr.y >= maze.len() as isize
    {
        return false;
    }

    // 2. On a wall
    if maze[curr.y as usize][curr.x as usize] == wall {
        return false;
    }

    // 3. On end
    if curr.x == end.x && curr.y == end.y {
        path.push(end.clone());
        return true;
    }

    // 4. On an already visited point
    if already_walked[curr.y as usize][curr.x as usize] {
        return false;
    }

    // Recursive Case
    // pre
    already_walked[curr.y as usize][curr.x as usize] = true;
    path.push(curr.clone());

    // recurse
    for (x, y) in DIRS {
        if walk(
            maze,
            wall,
            Point {
                x: curr.x + x,
                y: curr.y + y,
            },
            end.clone(),
            already_walked,
            path,
        ) {
            return true;
        };
    }

    // post
    path.pop();

    false
}

fn solve(maze: &Vec<Vec<&str>>, wall: &str, start: Point, end: Point) -> Vec<Point> {
    let mut already_walked = vec![vec![false; maze[0].len()]; maze.len()];
    let mut path = Vec::new();

    walk(maze, wall, start, end, &mut already_walked, &mut path);

    path
}

#[cfg(test)]
mod test {
    use super::*;

    // add further tests

    #[test]
    fn test_path_finding() {
        // TODO: make a more complexe maze (cf: The algo courses)
        #[rustfmt::skip]
        let maze = vec![
            vec!["#","#","#","#","#","E","#"],
            vec!["#"," "," "," "," "," ","#"],
            vec!["#","S","#","#","#","#","#"],
        ];

        assert_eq!(
            solve(&maze, "#", (1, 2).into(), (5, 0).into()),
            vec![
                Point::new(1, 2),
                Point::new(1, 1),
                Point::new(2, 1),
                Point::new(3, 1),
                Point::new(4, 1),
                Point::new(5, 1),
                Point::new(5, 0)
            ]
        );
    }
}
