use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

#[derive(Clone, Copy)]
pub enum Direction4 {
   North = 0b0001,
   South = 0b0010,
   East = 0b0100,
   West = 0b1000,
}

#[derive(Clone, Copy)]
pub struct Wall(u8);

impl Wall {
   pub const NONE: Self = Self(0);

   pub fn exists(&self) -> bool {
      self.0 != 0
   }
}

impl BitAnd<Direction4> for Wall {
   type Output = Self;

   fn bitand(self, rhs: Direction4) -> Self {
      Self(self.0 & rhs as u8)
   }
}

impl BitAndAssign<Direction4> for Wall {
   fn bitand_assign(&mut self, rhs: Direction4) {
      *self = *self & rhs
   }
}

impl BitOr<Direction4> for Wall {
   type Output = Self;

   fn bitor(self, rhs: Direction4) -> Self {
      Self(self.0 | rhs as u8)
   }
}

impl BitOrAssign<Direction4> for Wall {
   fn bitor_assign(&mut self, rhs: Direction4) {
      *self = *self | rhs
   }
}

impl BitXor<Direction4> for Wall {
   type Output = Self;

   fn bitxor(self, rhs: Direction4) -> Self {
      Self(self.0 ^ rhs as u8)
   }
}

impl BitXorAssign<Direction4> for Wall {
   fn bitxor_assign(&mut self, rhs: Direction4) {
      *self = *self ^ rhs
   }
}

impl Not for Wall {
   type Output = Self;

   fn not(self) -> Self {
      Self(!self.0)
   }
}

pub type Walls = Vec<Vec<Wall>>;
