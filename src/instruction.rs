use super::cpu::*;

const ON: bool = true;
const OFF: bool = false;

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

pub enum Addressing {

}

pub enum Interrupt {
  RESET,
  NMI,
  IRQ,
  BRK
}

impl Cpu {
  // todo: machineを引数で渡せるようにする
  pub fn interrupt(&mut self, inst: Interrupt) {
    match inst {
      Interrupt::RESET => {
        self.set_i_flag(ON);
        // TODO: PCの下位バイトを$FFFCから
        // 上位バイトを$FFFDからフェッチ
      },

      Interrupt::NMI => {
        self.set_b_flag(OFF);
        // TODO: PCの上位バイト、 下位バイト
        // ステータスレジスタを順にスタックへ格納
        self.set_i_flag(ON);
      },

      Interrupt::IRQ => {
        if self.read_i_flag() == 0 {
          self.set_b_flag(OFF);
          // TODO: PCの上位バイト、下位バイト、
          // ステータスレジスタを順にスタックへ格納
          self.set_i_flag(ON);
          // TODO: PCの下位バイトを$FFFEから、
          // 上位バイトを$FFFFからフェッチ
        }
      },

      Interrupt::BRK => {
        if self.read_i_flag() == 0 {
          self.set_b_flag(ON);
          self.pc += 1;
          // TODO: PCの上位バイト、下位バイト、
          // ステータスレジスタを順にスタックへ格納
          self.set_i_flag(ON);
          // TODO: PCの下位バイトを$FFFEから、
          // 上位バイトを$FFFFからフェッチ
        }
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

  // フラグ(n-bit目)の読み出し
  fn read_i_flag(&self) -> u8 {
    (self.p >> 3) & 1
  }

  // フラグ(n bit目)の操作
  fn set_i_flag(&mut self, stat: bool) {
    self.p = if stat {
      self.p | 0x04
    } else {
      self.p & (!0x04)
    };
  }

  fn set_b_flag(&mut self, stat: bool) {
    self.p = if stat {
      self.p | 0x08
    } else {
      self.p & (!0x08)
    };
  }
}
