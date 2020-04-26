use super::cpu::*;

pub enum Opcode {
  // transfer
  LDA,
  LDX,
  LDY,
  STA,
  STX,
  STY,
  TAX,
  TAY,
  TSX,
  TXA,
  TXS,
  TYA,
  // calculate
  ADC,
  AND,
  ASL,
  BIT,
  CMP,
  CPX,
  CPY,
  DEC,
  DEX,
  DEY,
  EOR,
  INC,
  INX,
  INY,
  LSR,
  ORA,
  ROL,
  ROR,
  SBC,
  // stack
  PHA,
  PHP,
  PLA,
  PLP,
  // jump
  JMP,
  JSR,
  RTS,
  RTI,
  // branch
  BCC,
  BCS,
  BEQ,
  BMI,
  BNE,
  BPL,
  BVC,
  BVS,
  // flag
  CLC,
  CLD,
  CLI,
  CLV,
  SEC,
  SED,
  SEI,
  // other
  BRK,
  NOP
}

pub enum Interrupt {
  RESET
}

impl Cpu {
  // todo: machineを引数で渡せるようにする
  pub fn interrupt(&mut self, inst: Interrupt) {
    match inst {
      Interrupt::RESET => {
        let p = self.p;
        self.p = self.p ^ 0x8;
        // todo: メモリ操作
        println!("RESET: \x1b[38;5;51m0x{:>08x} -> 0x{:>08x}\x1b[m", p, self.p);
      },

      _ => panic!("Unknown interrupt")
    }
  }

  pub fn exec(&mut self, inst: Opcode) {
    match inst {
      Opcode::ADC => {
      },

      _ => panic!("Unknown opcode")
    }
  }
}
