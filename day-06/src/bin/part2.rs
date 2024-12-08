use std::collections::{HashMap, HashSet};

#[derive(Debug, Eq, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
    Off,
}

type Position = (isize, isize);
type Puzzle = HashMap<Position, char>;

fn step(pos: &mut Position, dir: &mut Direction, places: &mut HashSet<Position>, puzzle: &Puzzle) -> () {
    match dir {
        Direction::North => {
            let try_position = (pos.0 - 1, pos.1);
            let item = puzzle.get(&try_position).unwrap_or(&'Z');
            match &item {
                '.' | '^' => {
                    *pos = try_position;
                    places.insert(try_position);
                }
                'Z' => *dir = Direction::Off,
                _ => *dir = Direction::East ,
            }
        }
        Direction::East => {
            let try_position = (pos.0, pos.1 + 1);
            let item = puzzle.get(&try_position).unwrap_or(&'Z');
            match &item {
                '.' | '^' => {
                    *pos = try_position;
                    places.insert(try_position);
                }
                'Z' => *dir = Direction::Off,
                _ =>  *dir = Direction::South,
            }
        }
        Direction::South => {
            let try_position = (pos.0 + 1, pos.1);
            let item = puzzle.get(&try_position).unwrap_or(&'Z');
            match &item {
                '.' | '^' => {
                    *pos = try_position;
                    places.insert(try_position);
                }
                'Z' => *dir = Direction::Off,
                _ =>  *dir = Direction::West,
            }
        }
        Direction::West => {
            let try_position = (pos.0, pos.1 - 1);
            let item = puzzle.get(&try_position).unwrap_or(&'Z');
            match &item {
                '.' | '^' => {
                    *pos = try_position;
                    places.insert(try_position);
                }
                'Z' => *dir = Direction::Off,
                _ =>  *dir = Direction::North,
            }
        }
        _ => (),
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
        assert_eq!(6, calc(data));
    }
}

