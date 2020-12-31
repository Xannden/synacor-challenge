#![allow(dead_code)]

use std::{
    collections::{HashMap, VecDeque},
    io::Write,
};
use std::{fs::File, writeln};

use itertools::Itertools;

fn main() {
    // coin_puzzle();
    // power_level();
    // orb_maze();
}

fn coin_puzzle() {
    let coins: [i64; 5] = [2, 3, 5, 7, 9];

    for p in coins.iter().permutations(5) {
        if p[0] + p[1] * (p[2].pow(2)) + (p[3].pow(3)) - p[4] == 399 {
            println!("Order is {:?}", p);
            return;
        }
    }
}

fn power_level() {
    //The anwser is 25734
    let child = std::thread::Builder::new()
        .stack_size(1024 * 1024 * 1024 * 8)
        .spawn(run)
        .unwrap();

    child.join().unwrap();

    fn run() {
        for i in 25500..u16::MAX {
            let mut cache = HashMap::new();
            let result = func(4, 1, i, &mut cache);

            if result == 6 {
                println!("Power Level is {}", i);
                break;
            } else if i % 500 == 0 {
                println!("i={} is {}", i, result);
            }
        }
    }
}

// Regs [4, 1, 3, 10, 101, 0, 0, 60]
fn func(m: u16, n: u16, r8: u16, cache: &mut HashMap<(u16, u16, u16), u16>) -> u16 {
    if let Some(val) = cache.get(&(m, n, r8)) {
        return *val;
    }

    let result = if m == 0 {
        // regs[0] = (regs[1] + 1) % 32768;
        (n + 1) % 32768
    } else if n == 0 {
        // regs[0] -= 1;
        // regs[1] = regs[7];
        func(m - 1, r8, r8, cache)
    // func(m - 1, 60)
    } else {
        // let temp = regs[0];
        // regs[1] -= 1;
        // regs[1] = regs[0];
        // regs[0] = temp;
        // regs[0] -= 1;
        func(m - 1, func(m, n - 1, r8, cache), r8, cache)
    };

    cache.insert((m, n, r8), result);

    result
}

enum Tile {
    Num(i64),
    Sub,
    Plus,
    Multi,
}

impl Tile {
    fn get_num(&self) -> i64 {
        match self {
            Tile::Num(val) => *val,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Move {
    North,
    South,
    East,
    West,
}

struct State {
    pos: (usize, usize),
    total: i64,
    moves: Vec<Move>,
}

impl State {
    fn is_exit(&self) -> bool {
        self.pos.0 == 3 && self.pos.1 == 0 && self.total == 30
    }

    fn next(&self) -> Vec<State> {
        if self.pos.0 == 3 && self.pos.1 == 0 {
            return Vec::new();
        }

        let mut next = Vec::new();

        if let Some(n) = self.north() {
            next.push(n);
        }

        if let Some(s) = self.south() {
            next.push(s);
        }

        if let Some(e) = self.east() {
            next.push(e);
        }

        if let Some(w) = self.west() {
            next.push(w);
        }

        next
    }

    fn north(&self) -> Option<State> {
        if self.pos.1 == 0 {
            None
        } else {
            Some(self.step(Move::North))
        }
    }

    fn south(&self) -> Option<State> {
        if self.pos.1 == 3 || (self.pos.0 == 0 && self.pos.1 == 2) {
            None
        } else {
            Some(self.step(Move::South))
        }
    }

    fn east(&self) -> Option<State> {
        if self.pos.0 == 3 {
            None
        } else {
            Some(self.step(Move::East))
        }
    }

    fn west(&self) -> Option<State> {
        if self.pos.0 == 0 || (self.pos.0 == 1 && self.pos.1 == 3) {
            None
        } else {
            Some(self.step(Move::West))
        }
    }

    fn step(&self, dir: Move) -> State {
        let next_pos = match dir {
            Move::North => (self.pos.0, self.pos.1 - 1),
            Move::South => (self.pos.0, self.pos.1 + 1),
            Move::East => (self.pos.0 + 1, self.pos.1),
            Move::West => (self.pos.0 - 1, self.pos.1),
        };

        let next_tile = &MAZE[next_pos.1][next_pos.0];

        let next_total = match MAZE[self.pos.1][self.pos.0] {
            Tile::Num(val) => self.total,
            Tile::Sub => self.total - next_tile.get_num(),
            Tile::Plus => self.total + next_tile.get_num(),
            Tile::Multi => self.total * next_tile.get_num(),
        };

        let mut moves = self.moves.clone();
        moves.push(dir);
        State {
            pos: next_pos,
            total: next_total,
            moves,
        }
    }
}

static MAZE: [[Tile; 4]; 4] = [
    [Tile::Multi, Tile::Num(8), Tile::Sub, Tile::Num(1)],
    [Tile::Num(4), Tile::Multi, Tile::Num(11), Tile::Multi],
    [Tile::Plus, Tile::Num(4), Tile::Sub, Tile::Num(18)],
    [Tile::Num(22), Tile::Sub, Tile::Num(9), Tile::Multi],
];

fn orb_maze() {
    let mut frontier = VecDeque::new();
    frontier.push_back(State {
        pos: (0, 3),
        total: 22,
        moves: Vec::new(),
    });

    while let Some(state) = frontier.pop_front() {
        if state.is_exit() {
            println!("Path found {:?}", state.moves);
            break;
        }

        for next in state.next() {
            frontier.push_back(next);
        }
    }
}
