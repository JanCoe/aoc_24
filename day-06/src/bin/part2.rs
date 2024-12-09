use std::collections::{HashMap, HashSet};

type Position = (isize, isize);
type Puzzle = HashMap<Position, char>;
type Places = HashSet<(Position, Direction)>;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
    Off,
}

impl Direction {
    fn turn_right(&mut self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            Direction::Off => Direction::Off,
        }
    }
}

fn new_position(position: &Position, direction: &Direction) -> Position {
    match direction {
        Direction::North => (position.0 - 1, position.1),
        Direction::East => (position.0, position.1 + 1),
        Direction::South => (position.0 + 1, position.1),
        Direction::West => (position.0, position.1 - 1),
        Direction::Off => (position.0, position.1),
    }
}

fn step(
    pos: &mut Position,
    dir: &mut Direction,
    places: &mut Places,
    first_time: &mut bool,
    puzzle: &Puzzle,
) {
    let try_position = new_position(pos, dir);
    let item = puzzle.get(&try_position).unwrap_or(&'Z');
    match &item {
        '.' | '^' => {
            *pos = try_position;
            *first_time = places.insert((try_position, dir.clone()));
        }
        'Z' => {
            *dir = Direction::Off;
            *pos = try_position;
            *first_time = true
        }
        _ => {
            *dir = dir.turn_right();
            *first_time = true
        }
    }
}

pub fn calc(data: &str) -> u16 {
    let mut puzzle: Puzzle = data
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(move |(col, ch)| ((row as isize, col as isize), ch))
        })
        .collect();

    let start_position = puzzle
        .iter()
        .filter(|(_, &ch)| ch == '^')
        .map(|(pos, _)| *pos)
        .next()
        .unwrap();

    let non_obstructions: Vec<Position> = puzzle
        .iter()
        .filter(|(_, &ch)| ch == '.')
        .map(|(pos, _)| *pos)
        .collect();

    let mut total = 0;
    for new_obstruction in non_obstructions {
        // this could be improved by only placing obstructions on the original path.
        puzzle.insert(new_obstruction, '#');

        let mut position = start_position.clone();
        let mut direction = Direction::North;
        let mut places = HashSet::new();
        places.insert((position, direction.clone()));
        let mut first_time = true;

        loop {
            step(
                &mut position,
                &mut direction,
                &mut places,
                &mut first_time,
                &puzzle,
            );

            if direction == Direction::Off {
                break;
            }

            if !first_time {
                total += 1;
                break;
            }
        }
        puzzle.insert(new_obstruction, '.');
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc() -> () {
        let data: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!(6, calc(data));
    }
}
