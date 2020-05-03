use super::machine;

#[derive(Debug, Copy, Clone)]
pub struct Cpu {
  // accumulator
  pub a: u8,
  // index
  pub x: u8,
  pub y: u8,
  // program counter
  pub pc: u16,
  // stack pointer
  pub sp: u16,
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
  pub p: u8
}

impl Cpu {
  pub fn new() -> Self {
    Self {
      a: 0,
      x: 0,
      y: 0,
      pc: 0,
      sp: 0xfe,
      p: 0x20
    }
  }

  pub fn push_stack(&mut self, machine: &mut machine::Machine, val: u8) {
    let addr = 0x100 + self.sp as u16;
    machine.store(addr as usize, val);
    self.sp -= 1;
  }

  pub fn pop_stack(&mut self, machine: &mut machine::Machine, addr: usize) -> u8 {
    self.sp += 1;
    machine.fetch(addr)
  }
}

