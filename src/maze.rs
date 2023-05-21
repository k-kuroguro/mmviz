use std::cmp::max;

use crate::wall::{Direction4, Wall, Walls};

#[derive(Clone, Copy)]
pub struct Pos {
   pub x: usize,
   pub y: usize,
}

impl Pos {
   pub const fn new(x: usize, y: usize) -> Self {
      Self { x, y }
   }
}

// 迷路は左下を原点とする.
//
// Y
// ^
// +-------+-------+-------+
// | (0,2) |   -   | (2,2) |
// +-------+-------+-------+
// |   |   |   /   |   |   |
// +-------+-------+-------+
// | (0,0) |   -   | (2,0) |
// +-------+-------+-------+--> X

#[derive(Clone)]
pub struct Maze {
   size: u8,
   walls: Walls,
   start: Pos,
   goals: Vec<Pos>,
}

impl Maze {
   pub fn size(&self) -> u8 {
      self.size
   }

   pub fn walls(&self) -> &Walls {
      &self.walls
   }

   pub fn start(&self) -> Pos {
      self.start
   }

   pub fn goals(&self) -> &Vec<Pos> {
      &self.goals
   }

   // File format: https://github.com/kerikun11/micromouse-maze-data
   pub fn from_str(s: &str) -> Self {
      // TODO: validation
      let size: u8 = max(
         (s.lines().count() / 2) as u8,
         (s.lines().next().unwrap_or("").len() / 4) as u8,
      );
      let mut walls: Walls = vec![vec![Wall::NONE; size as usize]; size as usize];
      let mut start = Pos::new(0, 0);
      let mut goals: Vec<Pos> = Vec::new();
      for (i, line) in s.lines().rev().enumerate() {
         if i % 2 == 0 {
            // +---+---+---+---+
            for (j, c) in line[2..].chars().step_by(4).enumerate() {
               if c == '-' {
                  if i / 2 < size as usize {
                     walls[j][i / 2] |= Direction4::South;
                  }
                  if i / 2 > 0 {
                     walls[j][i / 2 - 1] |= Direction4::North;
                  }
               }
            }
         } else {
            // |   |   | G |   |
            for (j, c) in line.chars().step_by(4).enumerate() {
               if c == '|' {
                  if j < size as usize {
                     walls[j][i / 2] |= Direction4::West;
                  }
                  if j > 0 {
                     walls[j - 1][i / 2] |= Direction4::East;
                  }
               }
            }
            for (j, c) in line[2..].chars().step_by(4).enumerate() {
               if c == 'S' {
                  start = Pos::new(j, i / 2);
               }
               if c == 'G' {
                  goals.push(Pos::new(j, i / 2));
               }
            }
         }
      }

      Maze {
         size,
         walls,
         start,
         goals,
      }
   }
}
