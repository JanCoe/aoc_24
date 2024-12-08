use std::collections::{HashMap, HashSet};

type Position = (isize, isize);
type Puzzle = HashMap<Position, char>;

#[derive(Debug, Eq, PartialEq)]
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


fn step(pos: &mut Position, dir: &mut Direction, places: &mut HashSet<Position>, puzzle: &Puzzle) -> () {
    let try_position = new_position(pos, dir);
    let item = puzzle.get(&try_position).unwrap_or(&'Z');
    match &item {
        '.' | '^' => {
            *pos = try_position;
            places.insert(try_position);
        }
        'Z' => *dir = Direction::Off,
        _ => *dir = dir.turn_right(),
    }
}

pub fn calc(data: &str) -> u16 {
    let puzzle: Puzzle = data
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(move |(col, ch)| ((row as isize, col as isize), ch))
        })
        .collect();

    let mut position = puzzle
        .iter()
        .filter(|(_, &ch)| ch == '^')
        .map(|(pos, _)| *pos)
        .next()
        .unwrap();

    let mut places = HashSet::new();
    places.insert(position);
    let mut direction = Direction::North;
    while direction != Direction::Off {
        step(&mut position, &mut direction, &mut places, &puzzle);
    };

    places.iter().count() as u16
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
        assert_eq!(41, calc(data));
    }
}
