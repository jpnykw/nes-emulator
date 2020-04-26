#[derive(Debug, Copy, Clone)]
pub struct Cpu {
  // accumulator
  pub a: i8,
  // index
  pub x: i8,
  pub y: i8,
  // program counter
  pub pc: i16,
  // stack pointer
  pub sp: i8,
  /*
   * processer status
   * 7: negative,
   * 6: overflow,
   * 4: break,
   * 3: decimal,
   * 2: interrupt,
   * 1: zero,
   * 0: carry
   */
  pub p: i8
}

impl Cpu {
  pub fn new() -> Self {
    Self {
      a: 0,
      x: 0,
      y: 0,
      pc: 0,
      sp: 0,
      p: 0x20
    }
  }
}

