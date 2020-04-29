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
  Implied,
  Accumulator,
  Immediate,
  Zeropage,
  ZeropageX,
  ZeropageY,
  Relative,
  Absolute,
  AbsoluteX,
  AbsoluteY,
  Indirect,
  IndirectX,
  IndirectY
}

pub enum Interrupt {
  RESET,
  NMI,
  IRQ,
  BRK
}

pub struct Instruction(Opcode, Addressing);

impl Cpu {
  // TODO: ROMのマシンコードを命令に変換する
  pub fn convert(code: u8) -> Instruction {
    match code {
      /// 転送命令
      // LDA
      0xa9 => Instruction(Opcode::LDA, Addressing::Immediate),
      0xa5 => Instruction(Opcode::LDA, Addressing::Zeropage),
      0xb5 => Instruction(Opcode::LDA, Addressing::ZeropageX),
      0xad => Instruction(Opcode::LDA, Addressing::Absolute),
      0xbd => Instruction(Opcode::LDA, Addressing::AbsoluteX),
      0xa1 => Instruction(Opcode::LDA, Addressing::AbsoluteY),
      0xb1 => Instruction(Opcode::LDA, Addressing::IndirectY),
      // LDX
      0xa2 => Instruction(Opcode::LDX, Addressing::Immediate),
      0xa6 => Instruction(Opcode::LDX, Addressing::Zeropage),
      0xb6 => Instruction(Opcode::LDX, Addressing::ZeropageY),
      0xae => Instruction(Opcode::LDX, Addressing::Absolute),
      0xbe => Instruction(Opcode::LDX, Addressing::AbsoluteY),
      // LDY
      0xa0 => Instruction(Opcode::LDY, Addressing::Immediate),
      0xa4 => Instruction(Opcode::LDY, Addressing::Zeropage),
      0xb4 => Instruction(Opcode::LDY, Addressing::ZeropageX),
      0xac => Instruction(Opcode::LDY, Addressing::Absolute),
      0xbc => Instruction(Opcode::LDY, Addressing::AbsoluteX),
      // STA
      0x85 => Instruction(Opcode::STA, Addressing::Zeropage),
      0x95 => Instruction(Opcode::STA, Addressing::ZeropageX),
      0x8d => Instruction(Opcode::STA, Addressing::Absolute),
      0x9d => Instruction(Opcode::STA, Addressing::AbsoluteX),
      0x99 => Instruction(Opcode::STA, Addressing::AbsoluteY),
      0x81 => Instruction(Opcode::STA, Addressing::IndirectX),
      0x91 => Instruction(Opcode::STA, Addressing::IndirectY),
      // STX
      0x86 => Instruction(Opcode::STX, Addressing::Zeropage),
      0x96 => Instruction(Opcode::STX, Addressing::ZeropageY),
      0x8e => Instruction(Opcode::STX, Addressing::Absolute),
      // STY
      0x84 => Instruction(Opcode::STY, Addressing::Zeropage),
      0x94 => Instruction(Opcode::STY, Addressing::ZeropageX),
      0x8c => Instruction(Opcode::STY, Addressing::Absolute),
      // TAX, TAY, TSX, TXA, TXS, TYA
      0xaa => Instruction(Opcode::TAX, Addressing::Implied),
      0xa8 => Instruction(Opcode::TAY, Addressing::Implied),
      0xba => Instruction(Opcode::TSX, Addressing::Implied),
      0x8a => Instruction(Opcode::TXA, Addressing::Implied),
      0x9a => Instruction(Opcode::TXS, Addressing::Implied),
      0x98 => Instruction(Opcode::TYA, Addressing::Implied),

      _ => panic!("Invalid machine code"),
    }
  }

  // TODO: machineを引数で渡せるようにする
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
