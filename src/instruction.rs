use super::cpu::*;
use super::machine;

const ON: bool = true;
const OFF: bool = false;

#[derive(Debug)]
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
  NOP,
  // unofficial (RMW instruction)
  DCP,
  ISC,
  RLA,
  RRA,
  SLO,
  SRE,
  ALR,
  ANC,
  ARR,
  AXS,
  LAX,
  SAX,
  SKB,
  IGN
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum Interrupt {
  RESET,
  NMI,
  IRQ,
  BRK
}

#[derive(Debug)]
pub struct Instruction(Opcode, Addressing);

/// # CPUの命令
///
/// マシンコードを命令とアドレッシングモードに変換します
///
/// ```rust
/// fn main() {
///   // Create CPU
///   let cpu = cpu::Cpu::new();
///   // Return (LDA, Immediate)
///   let inst = cpu.convert(0xa9);
/// }
/// ```
impl Cpu {
  // TODO: ROMのマシンコードを命令に変換する
  pub fn convert(self, code: u8) -> Instruction {
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
      0x51 => Instruction(Opcode::EOR, Addressing::IndirectY),
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

      /// その他
      // BRK
      0x00 => Instruction(Opcode::BRK, Addressing::Implied),
      // NOP
      0x1a => Instruction(Opcode::NOP, Addressing::Implied),
      0x3a => Instruction(Opcode::NOP, Addressing::Implied),
      0x5a => Instruction(Opcode::NOP, Addressing::Implied),
      0x7a => Instruction(Opcode::NOP, Addressing::Implied),
      0xda => Instruction(Opcode::NOP, Addressing::Implied),
      0xea => Instruction(Opcode::NOP, Addressing::Implied),
      0xfa => Instruction(Opcode::NOP, Addressing::Implied),
      // SKB
      0x80 => Instruction(Opcode::SKB, Addressing::Immediate),
      0x82 => Instruction(Opcode::SKB, Addressing::Immediate),
      0x89 => Instruction(Opcode::SKB, Addressing::Immediate),
      0xc2 => Instruction(Opcode::SKB, Addressing::Immediate),
      0xe2 => Instruction(Opcode::SKB, Addressing::Immediate),

      // unofficial_opcodesが必要らしい
      /// RMW 命令
      // DCP
      0xc3 => Instruction(Opcode::DCP, Addressing::IndirectX),
      0xc7 => Instruction(Opcode::DCP, Addressing::Zeropage),
      0xcf => Instruction(Opcode::DCP, Addressing::Absolute),
      0xd3 => Instruction(Opcode::DCP, Addressing::IndirectY),
      0xd7 => Instruction(Opcode::DCP, Addressing::ZeropageX),
      0xdb => Instruction(Opcode::DCP, Addressing::AbsoluteY),
      0xdf => Instruction(Opcode::DCP, Addressing::AbsoluteX),
      // ISC
      0xe3 => Instruction(Opcode::ISC, Addressing::IndirectX),
      0xe7 => Instruction(Opcode::ISC, Addressing::Zeropage),
      0xef => Instruction(Opcode::ISC, Addressing::Absolute),
      0xf3 => Instruction(Opcode::ISC, Addressing::IndirectY),
      0xf7 => Instruction(Opcode::ISC, Addressing::ZeropageX),
      0xfb => Instruction(Opcode::ISC, Addressing::AbsoluteX),
      0xff => Instruction(Opcode::ISC, Addressing::AbsoluteX),
      // RLA
      0x23 => Instruction(Opcode::RLA, Addressing::IndirectX),
      0x27 => Instruction(Opcode::RLA, Addressing::Zeropage),
      0x2f => Instruction(Opcode::RLA, Addressing::Absolute),
      0x33 => Instruction(Opcode::RLA, Addressing::IndirectY),
      0x37 => Instruction(Opcode::RLA, Addressing::ZeropageX),
      0x3b => Instruction(Opcode::RLA, Addressing::AbsoluteY),
      0x3f => Instruction(Opcode::RLA, Addressing::AbsoluteX),
      // RRA
      0x63 => Instruction(Opcode::RRA, Addressing::IndirectX),
      0x67 => Instruction(Opcode::RRA, Addressing::Zeropage),
      0x6f => Instruction(Opcode::RRA, Addressing::Absolute),
      0x73 => Instruction(Opcode::RRA, Addressing::IndirectY),
      0x77 => Instruction(Opcode::RRA, Addressing::ZeropageX),
      0x7b => Instruction(Opcode::RRA, Addressing::AbsoluteY),
      0x7f => Instruction(Opcode::RRA, Addressing::AbsoluteX),
      // SLO
      0x03 => Instruction(Opcode::SLO, Addressing::IndirectX),
      0x07 => Instruction(Opcode::SLO, Addressing::Zeropage),
      0x0f => Instruction(Opcode::SLO, Addressing::Absolute),
      0x13 => Instruction(Opcode::SLO, Addressing::IndirectY),
      0x17 => Instruction(Opcode::SLO, Addressing::ZeropageX),
      0x1b => Instruction(Opcode::SLO, Addressing::AbsoluteY),
      0x1f => Instruction(Opcode::SLO, Addressing::AbsoluteX),
      // SRE
      0x43 => Instruction(Opcode::SRE, Addressing::IndirectX),
      0x47 => Instruction(Opcode::SRE, Addressing::Zeropage),
      0x4f => Instruction(Opcode::SRE, Addressing::Absolute),
      0x53 => Instruction(Opcode::SRE, Addressing::IndirectY),
      0x57 => Instruction(Opcode::SRE, Addressing::ZeropageX),
      0x5b => Instruction(Opcode::SRE, Addressing::AbsoluteY),
      0x5f => Instruction(Opcode::SRE, Addressing::AbsoluteX),

      /// 結合演算子
      0x4b => Instruction(Opcode::ALR, Addressing::Immediate),
      0x0b => Instruction(Opcode::ANC, Addressing::Immediate),
      0x6b => Instruction(Opcode::ARR, Addressing::Immediate),
      0xcb => Instruction(Opcode::AXS, Addressing::Immediate),
      // LAX
      0xa3 => Instruction(Opcode::LAX, Addressing::IndirectX),
      0xa7 => Instruction(Opcode::LAX, Addressing::Zeropage),
      0xaf => Instruction(Opcode::LAX, Addressing::Absolute),
      0xb3 => Instruction(Opcode::LAX, Addressing::IndirectY),
      0xb7 => Instruction(Opcode::LAX, Addressing::ZeropageY),
      0xbf => Instruction(Opcode::LAX, Addressing::AbsoluteY),
      // SAX
      0x83 => Instruction(Opcode::SAX, Addressing::IndirectX),
      0x87 => Instruction(Opcode::SAX, Addressing::Zeropage),
      0x8f => Instruction(Opcode::SAX, Addressing::Absolute),
      0x97 => Instruction(Opcode::SAX, Addressing::ZeropageY),
      // IGN
      0x0c => Instruction(Opcode::IGN, Addressing::Absolute),

      0x1c => Instruction(Opcode::IGN, Addressing::AbsoluteX),
      0x3c => Instruction(Opcode::IGN, Addressing::AbsoluteX),
      0x5c => Instruction(Opcode::IGN, Addressing::AbsoluteX),
      0x7c => Instruction(Opcode::IGN, Addressing::AbsoluteX),
      0xdc => Instruction(Opcode::IGN, Addressing::AbsoluteX),
      0xfc => Instruction(Opcode::IGN, Addressing::AbsoluteX),

      0x04 => Instruction(Opcode::IGN, Addressing::Zeropage),
      0x44 => Instruction(Opcode::IGN, Addressing::Zeropage),
      0x64 => Instruction(Opcode::IGN, Addressing::Zeropage),

      0x14 => Instruction(Opcode::IGN, Addressing::ZeropageX),
      0x34 => Instruction(Opcode::IGN, Addressing::ZeropageX),
      0x54 => Instruction(Opcode::IGN, Addressing::ZeropageX),
      0x74 => Instruction(Opcode::IGN, Addressing::ZeropageX),
      0xd4 => Instruction(Opcode::IGN, Addressing::ZeropageX),
      0xf4 => Instruction(Opcode::IGN, Addressing::ZeropageX),

      // Unknown code となったものをNOPとしておく
      0x02 => Instruction(Opcode::NOP, Addressing::Immediate),
      0x12 => Instruction(Opcode::NOP, Addressing::Immediate),
      0x22 => Instruction(Opcode::NOP, Addressing::Immediate),
      0x32 => Instruction(Opcode::NOP, Addressing::Immediate),
      0x42 => Instruction(Opcode::NOP, Addressing::Immediate),
      0x52 => Instruction(Opcode::NOP, Addressing::Immediate),
      0x62 => Instruction(Opcode::NOP, Addressing::Immediate),
      0x72 => Instruction(Opcode::NOP, Addressing::Immediate),
      0x92 => Instruction(Opcode::NOP, Addressing::Immediate),

      // TODO: たまに未知の値を検出するとパニックするのでNOPで対応
      // TODO: 実装方法が正しいのかは不明 [要検証]
      // _ => panic!(format!("Unknown code 0x{:>02x}", code)),
      _ => {
        let text = format!("Unknown code 0x{:>02x}", code);

        // 本来はすべてpanicさせたい
        let mode = false;
        if mode { panic!(text); }

        // 仮で通す
        // println!("{}", text);
        Instruction(Opcode::NOP, Addressing::Immediate)
      }
    }
  }

  pub fn interrupt(&mut self, machine: &mut machine::Machine, inst: Interrupt) {
    match inst {
      Interrupt::RESET => {
        self.set_i_flag(ON);
        // TODO: PCの下位バイトを$FFFCから
        // 上位バイトを$FFFDからフェッチ

      },

      Interrupt::NMI => {
        self.set_b_flag(OFF);
        self.push_stack(machine, (self.pc >> 8) as u8);
        self.push_stack(machine, (self.pc & 255) as u8);
        self.push_stack(machine, self.p);
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
