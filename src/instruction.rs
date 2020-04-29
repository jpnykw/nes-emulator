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

      /// 算術命令
      // ADC
      0x69 => Instruction(Opcode::ADC, Addressing::Immediate),
      0x65 => Instruction(Opcode::ADC, Addressing::Zeropage),
      0x75 => Instruction(Opcode::ADC, Addressing::ZeropageX),
      0x6d => Instruction(Opcode::ADC, Addressing::Absolute),
      0x7d => Instruction(Opcode::ADC, Addressing::AbsoluteX),
      0x79 => Instruction(Opcode::ADC, Addressing::AbsoluteY),
      0x61 => Instruction(Opcode::ADC, Addressing::IndirectX),
      0x71 => Instruction(Opcode::ADC, Addressing::IndirectY),
      // AND
      0x29 => Instruction(Opcode::AND, Addressing::Immediate),
      0x25 => Instruction(Opcode::AND, Addressing::Zeropage),
      0x35 => Instruction(Opcode::AND, Addressing::ZeropageX),
      0x2d => Instruction(Opcode::AND, Addressing::Absolute),
      0x3d => Instruction(Opcode::AND, Addressing::AbsoluteX),
      0x39 => Instruction(Opcode::AND, Addressing::AbsoluteY),
      0x21 => Instruction(Opcode::AND, Addressing::IndirectX),
      0x31 => Instruction(Opcode::AND, Addressing::IndirectY),
      // ASL
      0x0a => Instruction(Opcode::ASL, Addressing::Accumulator),
      0x06 => Instruction(Opcode::ASL, Addressing::Zeropage),
      0x16 => Instruction(Opcode::ASL, Addressing::ZeropageX),
      0x0e => Instruction(Opcode::ASL, Addressing::Absolute),
      0x1e => Instruction(Opcode::ASL, Addressing::AbsoluteX),
      // BIT
      0x24 => Instruction(Opcode::BIT, Addressing::Zeropage),
      0x2c => Instruction(Opcode::BIT, Addressing::Absolute),
      // CMP
      0xc9 => Instruction(Opcode::CMP, Addressing::Immediate),
      0xc5 => Instruction(Opcode::CMP, Addressing::Zeropage),
      0xd5 => Instruction(Opcode::CMP, Addressing::ZeropageX),
      0xcd => Instruction(Opcode::CMP, Addressing::Absolute),
      0xdd => Instruction(Opcode::CMP, Addressing::AbsoluteX),
      0xd9 => Instruction(Opcode::CMP, Addressing::AbsoluteY),
      0xc1 => Instruction(Opcode::CMP, Addressing::IndirectX),
      0xd1 => Instruction(Opcode::CMP, Addressing::IndirectY),
      // CPX
      0xe0 => Instruction(Opcode::CPX, Addressing::Immediate),
      0xe4 => Instruction(Opcode::CPX, Addressing::Zeropage),
      0xec => Instruction(Opcode::CPX, Addressing::Absolute),
      // CPY
      0xc0 => Instruction(Opcode::CPX, Addressing::Immediate),
      0xc4 => Instruction(Opcode::CPX, Addressing::Zeropage),
      0xcc => Instruction(Opcode::CPX, Addressing::Absolute),
      // DEC
      0xc6 => Instruction(Opcode::DEC, Addressing::Zeropage),
      0xd6 => Instruction(Opcode::DEC, Addressing::ZeropageX),
      0xce => Instruction(Opcode::DEC, Addressing::Absolute),
      0xde => Instruction(Opcode::DEC, Addressing::AbsoluteX),
      // DEX, DEY
      0xca => Instruction(Opcode::DEX, Addressing::Implied),
      0x88 => Instruction(Opcode::DEY, Addressing::Implied),
      // EOR
      0x49 => Instruction(Opcode::EOR, Addressing::Immediate),
      0x45 => Instruction(Opcode::EOR, Addressing::Zeropage),
      0x55 => Instruction(Opcode::EOR, Addressing::ZeropageX),
      0x4d => Instruction(Opcode::EOR, Addressing::Absolute),
      0x5d => Instruction(Opcode::EOR, Addressing::AbsoluteX),
      0x59 => Instruction(Opcode::EOR, Addressing::AbsoluteY),
      0x41 => Instruction(Opcode::EOR, Addressing::IndirectX),
      0x41 => Instruction(Opcode::EOR, Addressing::IndirectY),
      // INC
      0xe6 => Instruction(Opcode::INC, Addressing::Zeropage),
      0xf6 => Instruction(Opcode::INC, Addressing::ZeropageX),
      0xee => Instruction(Opcode::INC, Addressing::Absolute),
      0xfe => Instruction(Opcode::INC, Addressing::AbsoluteX),
      // INX, INY
      0xe8 => Instruction(Opcode::INX, Addressing::Implied),
      0xc8 => Instruction(Opcode::INY, Addressing::Implied),
      // LSR
      0x4a => Instruction(Opcode::LSR, Addressing::Accumulator),
      0x46 => Instruction(Opcode::LSR, Addressing::Zeropage),
      0x56 => Instruction(Opcode::LSR, Addressing::ZeropageX),
      0x4e => Instruction(Opcode::LSR, Addressing::Absolute),
      0x5e => Instruction(Opcode::LSR, Addressing::AbsoluteX),
      // ORA
      0x09 => Instruction(Opcode::ORA, Addressing::Immediate),
      0x05 => Instruction(Opcode::ORA, Addressing::Zeropage),
      0x15 => Instruction(Opcode::ORA, Addressing::ZeropageX),
      0x0d => Instruction(Opcode::ORA, Addressing::Absolute),
      0x1d => Instruction(Opcode::ORA, Addressing::AbsoluteX),
      0x19 => Instruction(Opcode::ORA, Addressing::AbsoluteY),
      0x01 => Instruction(Opcode::ORA, Addressing::IndirectX),
      0x11 => Instruction(Opcode::ORA, Addressing::IndirectY),
      // ROL
      0x2a => Instruction(Opcode::ROL, Addressing::Accumulator),
      0x26 => Instruction(Opcode::ROL, Addressing::Zeropage),
      0x36 => Instruction(Opcode::ROL, Addressing::ZeropageX),
      0x2e => Instruction(Opcode::ROL, Addressing::Absolute),
      0x3e => Instruction(Opcode::ROL, Addressing::AbsoluteX),
      // ROR
      0x6a => Instruction(Opcode::ROR, Addressing::Accumulator),
      0x66 => Instruction(Opcode::ROR, Addressing::Zeropage),
      0x76 => Instruction(Opcode::ROR, Addressing::ZeropageX),
      0x6e => Instruction(Opcode::ROR, Addressing::Absolute),
      0x7e => Instruction(Opcode::ROR, Addressing::AbsoluteX),
      // SBC
      0xe9 => Instruction(Opcode::SBC, Addressing::Immediate),
      0xe5 => Instruction(Opcode::SBC, Addressing::Zeropage),
      0xf5 => Instruction(Opcode::SBC, Addressing::ZeropageX),
      0xed => Instruction(Opcode::SBC, Addressing::Absolute),
      0xfd => Instruction(Opcode::SBC, Addressing::AbsoluteX),
      0xf9 => Instruction(Opcode::SBC, Addressing::AbsoluteY),
      0xe1 => Instruction(Opcode::SBC, Addressing::IndirectX),
      0xf1 => Instruction(Opcode::SBC, Addressing::IndirectY),

      /// スタック命令
      // PHA, PHP, PLA, PLP
      0x48 => Instruction(Opcode::PHA, Addressing::Implied),
      0x08 => Instruction(Opcode::PHP, Addressing::Implied),
      0x68 => Instruction(Opcode::PLA, Addressing::Implied),
      0x28 => Instruction(Opcode::PLP, Addressing::Implied),

      /// ジャンプ命令
      // JMP
      0x4c => Instruction(Opcode::JMP, Addressing::Absolute),
      0x6c => Instruction(Opcode::JMP, Addressing::Indirect),
      // JSR, RTS, RTI
      0x20 => Instruction(Opcode::JSR, Addressing::Absolute),
      0x60 => Instruction(Opcode::RTS, Addressing::Implied),
      0x40 => Instruction(Opcode::RTI, Addressing::Implied),

      /// 分岐命令
      // BCC, BCS, BEQ, BMI, BNE, BPL, BVC, BVS
      0x90 => Instruction(Opcode::BCC, Addressing::Relative),
      0xb0 => Instruction(Opcode::BCS, Addressing::Relative),
      0xf0 => Instruction(Opcode::BEQ, Addressing::Relative),
      0x30 => Instruction(Opcode::BMI, Addressing::Relative),
      0xd0 => Instruction(Opcode::BNE, Addressing::Relative),
      0x10 => Instruction(Opcode::BPL, Addressing::Relative),
      0x50 => Instruction(Opcode::BVC, Addressing::Relative),
      0x70 => Instruction(Opcode::BVS, Addressing::Relative),

      /// フラグ変更命令
      // CLC, CLD, CLI, CLV, SEC, SED, SEI
      0x18 => Instruction(Opcode::CLC, Addressing::Implied),
      0xd8 => Instruction(Opcode::CLD, Addressing::Implied),
      0x58 => Instruction(Opcode::CLI, Addressing::Implied),
      0xb8 => Instruction(Opcode::CLV, Addressing::Implied),
      0x38 => Instruction(Opcode::SEC, Addressing::Implied),
      0xf8 => Instruction(Opcode::SED, Addressing::Implied),
      0x78 => Instruction(Opcode::SEI, Addressing::Implied),

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
