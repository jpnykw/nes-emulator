use super::machine;

#[derive(Debug, Copy, Clone)]
pub struct Cpu {
  /// Accumulator
  pub a: u8,
  /// Index register
  pub x: u8,
  pub y: u8,
  /// Program counter
  pub pc: u16,
  /// Stack pointer
  pub sp: u16,
  ///  Processer status register
  ///
  /// | Bit | Name | Description |
  /// | - | - | - |
  /// | 7 | **N** (Negative) | This is set if the result of an operation is negative, cleared if positive.
  /// | 6 | **V** (oVerflow) | When an arithmetic operation produces a result too large to be represented in a byte, V is set.
  /// | 5 | - | Not used. Supposed to be logical 1 at all times. |
  /// | 4 | **B** (Break) | This is set when a software interrupt (BRK instruction) is executed.
  /// | 3 | **D** (Decimal) | This is the decimal mode status flag. When set, and an Add with Carry or Subtract with Carry instruction is
  /// | | | executed, the source values are treated as valid BCD (Binary Coded Decimal, eg. 0x00-0x99 = 0-99) numbers.
  /// | | | The result generated is also a BCD number.
  /// | 2 | **I** (Interrupt) | This is an interrupt `enable/disable` flag.
  /// | | | If it is set, interrupts are disabled. If it is cleared, interrupts are enabled.
  /// | 1 | **Z** (Zero) | This is set to 1 when any arithmetic or logical operation produces a zero result,
  /// | | | and is set to 0 if the result is non-zero.
  /// | 0 | **C** (Carry) | This holds the carry out of the most significant bit in any arithmetic operation.
  /// | | | In subtraction operations however, this flag is cleared - set to 0 - if a borrow is required,
  /// | | | set to 1 - if no borrow is required. The carry flag is also used in shift and rotate logical operations.
  ///
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

