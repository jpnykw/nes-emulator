 struct Cpu {
  // accumulator
  a: i8,
  // index
  x: i8, y: i8,
  // program counter
  pc: i16,
  // stack pointer
  sp: i8,
  /*
   * processer status
   * 7: Negative,
   * 6: oVerflow,
   * 4: Break,
   * 3: Decimal,
   * 2: Interrupt,
   * 1: Zero,
   * 0: Carry
   */
  p: i8
}

impl Cpu {
  fn new() -> Self {
    Self {
      a: 0,
      x: 0, y: 0,
      pc: 0,
      sp: 0,
      p: 0
    }
  }
}

