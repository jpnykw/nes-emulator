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

#[derive(Debug, Copy, Clone, PartialEq)]
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
pub struct Instruction(u8, Opcode, Addressing); // 頭はCycle数

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
      0xa9 => Instruction(2, Opcode::LDA, Addressing::Immediate),
      0xa5 => Instruction(3, Opcode::LDA, Addressing::Zeropage),
      0xb5 => Instruction(4, Opcode::LDA, Addressing::ZeropageX),
      0xad => Instruction(4, Opcode::LDA, Addressing::Absolute),
      0xbd => Instruction(4, Opcode::LDA, Addressing::AbsoluteX),
      0xb9 => Instruction(4, Opcode::LDA, Addressing::AbsoluteY),
      0xa1 => Instruction(5, Opcode::LDA, Addressing::IndirectX),
      0xb1 => Instruction(6, Opcode::LDA, Addressing::IndirectY),
      // LDX
      0xa2 => Instruction(2, Opcode::LDX, Addressing::Immediate),
      0xa6 => Instruction(3, Opcode::LDX, Addressing::Zeropage),
      0xb6 => Instruction(4, Opcode::LDX, Addressing::ZeropageY),
      0xae => Instruction(4, Opcode::LDX, Addressing::Absolute),
      0xbe => Instruction(4, Opcode::LDX, Addressing::AbsoluteY),
      // LDY
      0xa0 => Instruction(2, Opcode::LDY, Addressing::Immediate),
      0xa4 => Instruction(3, Opcode::LDY, Addressing::Zeropage),
      0xb4 => Instruction(4, Opcode::LDY, Addressing::ZeropageX),
      0xac => Instruction(4, Opcode::LDY, Addressing::Absolute),
      0xbc => Instruction(4, Opcode::LDY, Addressing::AbsoluteX),
      // STA
      0x85 => Instruction(3, Opcode::STA, Addressing::Zeropage),
      0x95 => Instruction(4, Opcode::STA, Addressing::ZeropageX),
      0x8d => Instruction(4, Opcode::STA, Addressing::Absolute),
      0x9d => Instruction(5, Opcode::STA, Addressing::AbsoluteX),
      0x99 => Instruction(5, Opcode::STA, Addressing::AbsoluteY),
      0x81 => Instruction(6, Opcode::STA, Addressing::IndirectX),
      0x91 => Instruction(6, Opcode::STA, Addressing::IndirectY),
      // STX
      0x86 => Instruction(3, Opcode::STX, Addressing::Zeropage),
      0x96 => Instruction(4, Opcode::STX, Addressing::ZeropageY),
      0x8e => Instruction(4, Opcode::STX, Addressing::Absolute),
      // STY
      0x84 => Instruction(3, Opcode::STY, Addressing::Zeropage),
      0x94 => Instruction(4, Opcode::STY, Addressing::ZeropageX),
      0x8c => Instruction(4, Opcode::STY, Addressing::Absolute),
      // TAX, TAY, TSX, TXA, TXS, TYA
      0xaa => Instruction(2, Opcode::TAX, Addressing::Implied),
      0xa8 => Instruction(2, Opcode::TAY, Addressing::Implied),
      0xba => Instruction(2, Opcode::TSX, Addressing::Implied),
      0x8a => Instruction(2, Opcode::TXA, Addressing::Implied),
      0x9a => Instruction(2, Opcode::TXS, Addressing::Implied),
      0x98 => Instruction(2, Opcode::TYA, Addressing::Implied),

      /// 算術命令
      // ADC
      0x69 => Instruction(2, Opcode::ADC, Addressing::Immediate),
      0x65 => Instruction(3, Opcode::ADC, Addressing::Zeropage),
      0x75 => Instruction(4, Opcode::ADC, Addressing::ZeropageX),
      0x6d => Instruction(4, Opcode::ADC, Addressing::Absolute),
      0x7d => Instruction(4, Opcode::ADC, Addressing::AbsoluteX),
      0x79 => Instruction(4, Opcode::ADC, Addressing::AbsoluteY),
      0x61 => Instruction(6, Opcode::ADC, Addressing::IndirectX),
      0x71 => Instruction(5, Opcode::ADC, Addressing::IndirectY),
      // AND
      0x29 => Instruction(2, Opcode::AND, Addressing::Immediate),
      0x25 => Instruction(3, Opcode::AND, Addressing::Zeropage),
      0x35 => Instruction(4, Opcode::AND, Addressing::ZeropageX),
      0x2d => Instruction(4, Opcode::AND, Addressing::Absolute),
      0x3d => Instruction(4, Opcode::AND, Addressing::AbsoluteX),
      0x39 => Instruction(4, Opcode::AND, Addressing::AbsoluteY),
      0x21 => Instruction(6, Opcode::AND, Addressing::IndirectX),
      0x31 => Instruction(5, Opcode::AND, Addressing::IndirectY),
      // ASL
      0x0a => Instruction(2, Opcode::ASL, Addressing::Accumulator),
      0x06 => Instruction(5, Opcode::ASL, Addressing::Zeropage),
      0x16 => Instruction(6, Opcode::ASL, Addressing::ZeropageX),
      0x0e => Instruction(6, Opcode::ASL, Addressing::Absolute),
      0x1e => Instruction(7, Opcode::ASL, Addressing::AbsoluteX),
      // BIT
      0x24 => Instruction(3, Opcode::BIT, Addressing::Zeropage),
      0x2c => Instruction(4, Opcode::BIT, Addressing::Absolute),
      // CMP
      0xc9 => Instruction(2, Opcode::CMP, Addressing::Immediate),
      0xc5 => Instruction(3, Opcode::CMP, Addressing::Zeropage),
      0xd5 => Instruction(4, Opcode::CMP, Addressing::ZeropageX),
      0xcd => Instruction(4, Opcode::CMP, Addressing::Absolute),
      0xdd => Instruction(4, Opcode::CMP, Addressing::AbsoluteX),
      0xd9 => Instruction(4, Opcode::CMP, Addressing::AbsoluteY),
      0xc1 => Instruction(6, Opcode::CMP, Addressing::IndirectX),
      0xd1 => Instruction(5, Opcode::CMP, Addressing::IndirectY),
      // CPX
      0xe0 => Instruction(2, Opcode::CPX, Addressing::Immediate),
      0xe4 => Instruction(3, Opcode::CPX, Addressing::Zeropage),
      0xec => Instruction(4, Opcode::CPX, Addressing::Absolute),
      // CPY
      0xc0 => Instruction(2, Opcode::CPX, Addressing::Immediate),
      0xc4 => Instruction(3, Opcode::CPX, Addressing::Zeropage),
      0xcc => Instruction(4, Opcode::CPX, Addressing::Absolute),
      // DEC
      0xc6 => Instruction(5, Opcode::DEC, Addressing::Zeropage),
      0xd6 => Instruction(6, Opcode::DEC, Addressing::ZeropageX),
      0xce => Instruction(6, Opcode::DEC, Addressing::Absolute),
      0xde => Instruction(7, Opcode::DEC, Addressing::AbsoluteX),
      // DEX, DEY
      0xca => Instruction(2, Opcode::DEX, Addressing::Implied),
      0x88 => Instruction(2, Opcode::DEY, Addressing::Implied),
      // EOR
      0x49 => Instruction(2, Opcode::EOR, Addressing::Immediate),
      0x45 => Instruction(3, Opcode::EOR, Addressing::Zeropage),
      0x55 => Instruction(4, Opcode::EOR, Addressing::ZeropageX),
      0x4d => Instruction(4, Opcode::EOR, Addressing::Absolute),
      0x5d => Instruction(4, Opcode::EOR, Addressing::AbsoluteX),
      0x59 => Instruction(4, Opcode::EOR, Addressing::AbsoluteY),
      0x41 => Instruction(6, Opcode::EOR, Addressing::IndirectX),
      0x51 => Instruction(5, Opcode::EOR, Addressing::IndirectY),
      // INC
      0xe6 => Instruction(5, Opcode::INC, Addressing::Zeropage),
      0xf6 => Instruction(6, Opcode::INC, Addressing::ZeropageX),
      0xee => Instruction(6, Opcode::INC, Addressing::Absolute),
      0xfe => Instruction(7, Opcode::INC, Addressing::AbsoluteX),
      // INX, INY
      0xe8 => Instruction(2, Opcode::INX, Addressing::Implied),
      0xc8 => Instruction(2, Opcode::INY, Addressing::Implied),
      // LSR
      0x4a => Instruction(2, Opcode::LSR, Addressing::Accumulator),
      0x46 => Instruction(5, Opcode::LSR, Addressing::Zeropage),
      0x56 => Instruction(6, Opcode::LSR, Addressing::ZeropageX),
      0x4e => Instruction(6, Opcode::LSR, Addressing::Absolute),
      0x5e => Instruction(7, Opcode::LSR, Addressing::AbsoluteX),
      // ORA
      0x09 => Instruction(2, Opcode::ORA, Addressing::Immediate),
      0x05 => Instruction(3, Opcode::ORA, Addressing::Zeropage),
      0x15 => Instruction(4, Opcode::ORA, Addressing::ZeropageX),
      0x0d => Instruction(4, Opcode::ORA, Addressing::Absolute),
      0x1d => Instruction(4, Opcode::ORA, Addressing::AbsoluteX),
      0x19 => Instruction(4, Opcode::ORA, Addressing::AbsoluteY),
      0x01 => Instruction(6, Opcode::ORA, Addressing::IndirectX),
      0x11 => Instruction(5, Opcode::ORA, Addressing::IndirectY),
      // ROL
      0x2a => Instruction(2, Opcode::ROL, Addressing::Accumulator),
      0x26 => Instruction(5, Opcode::ROL, Addressing::Zeropage),
      0x36 => Instruction(6, Opcode::ROL, Addressing::ZeropageX),
      0x2e => Instruction(6, Opcode::ROL, Addressing::Absolute),
      0x3e => Instruction(7, Opcode::ROL, Addressing::AbsoluteX),
      // ROR
      0x6a => Instruction(2, Opcode::ROR, Addressing::Accumulator),
      0x66 => Instruction(5, Opcode::ROR, Addressing::Zeropage),
      0x76 => Instruction(6, Opcode::ROR, Addressing::ZeropageX),
      0x6e => Instruction(6, Opcode::ROR, Addressing::Absolute),
      0x7e => Instruction(7, Opcode::ROR, Addressing::AbsoluteX),
      // SBC
      0xe9 => Instruction(2, Opcode::SBC, Addressing::Immediate),
      0xe5 => Instruction(3, Opcode::SBC, Addressing::Zeropage),
      0xf5 => Instruction(4, Opcode::SBC, Addressing::ZeropageX),
      0xed => Instruction(4, Opcode::SBC, Addressing::Absolute),
      0xfd => Instruction(4, Opcode::SBC, Addressing::AbsoluteX),
      0xf9 => Instruction(4, Opcode::SBC, Addressing::AbsoluteY),
      0xe1 => Instruction(6, Opcode::SBC, Addressing::IndirectX),
      0xf1 => Instruction(5, Opcode::SBC, Addressing::IndirectY),

      /// スタック命令
      // PHA, PHP, PLA, PLP
      0x48 => Instruction(3, Opcode::PHA, Addressing::Implied),
      0x08 => Instruction(3, Opcode::PHP, Addressing::Implied),
      0x68 => Instruction(4, Opcode::PLA, Addressing::Implied),
      0x28 => Instruction(4, Opcode::PLP, Addressing::Implied),

      /// ジャンプ命令
      // JMP
      0x4c => Instruction(3, Opcode::JMP, Addressing::Absolute),
      0x6c => Instruction(5, Opcode::JMP, Addressing::Indirect),
      // JSR, RTS, RTI
      0x20 => Instruction(6, Opcode::JSR, Addressing::Absolute),
      0x60 => Instruction(6, Opcode::RTS, Addressing::Implied),
      0x40 => Instruction(6, Opcode::RTI, Addressing::Implied),

      /// 分岐命令
      // BCC, BCS, BEQ, BMI, BNE, BPL, BVC, BVS
      0x90 => Instruction(2, Opcode::BCC, Addressing::Relative),
      0xb0 => Instruction(2, Opcode::BCS, Addressing::Relative),
      0xf0 => Instruction(2, Opcode::BEQ, Addressing::Relative),
      0x30 => Instruction(2, Opcode::BMI, Addressing::Relative),
      0xd0 => Instruction(2, Opcode::BNE, Addressing::Relative),
      0x10 => Instruction(2, Opcode::BPL, Addressing::Relative),
      0x50 => Instruction(2, Opcode::BVC, Addressing::Relative),
      0x70 => Instruction(2, Opcode::BVS, Addressing::Relative),

      /// フラグ変更命令
      // CLC, CLD, CLI, CLV, SEC, SED, SEI
      0x18 => Instruction(2, Opcode::CLC, Addressing::Implied),
      0xd8 => Instruction(2, Opcode::CLD, Addressing::Implied),
      0x58 => Instruction(2, Opcode::CLI, Addressing::Implied),
      0xb8 => Instruction(2, Opcode::CLV, Addressing::Implied),
      0x38 => Instruction(2, Opcode::SEC, Addressing::Implied),
      0xf8 => Instruction(2, Opcode::SED, Addressing::Implied),
      0x78 => Instruction(2, Opcode::SEI, Addressing::Implied),

      /// その他
      // BRK
      0x00 => Instruction(7, Opcode::BRK, Addressing::Implied),
      // NOP
      0x1a => Instruction(1, Opcode::NOP, Addressing::Implied),
      0x3a => Instruction(1, Opcode::NOP, Addressing::Implied),
      0x5a => Instruction(1, Opcode::NOP, Addressing::Implied),
      0x7a => Instruction(1, Opcode::NOP, Addressing::Implied),
      0xda => Instruction(1, Opcode::NOP, Addressing::Implied),
      0xea => Instruction(1, Opcode::NOP, Addressing::Implied),
      0xfa => Instruction(1, Opcode::NOP, Addressing::Implied),
      // SKB
      0x80 => Instruction(2, Opcode::SKB, Addressing::Immediate),
      0x82 => Instruction(2, Opcode::SKB, Addressing::Immediate),
      0x89 => Instruction(2, Opcode::SKB, Addressing::Immediate),
      0xc2 => Instruction(2, Opcode::SKB, Addressing::Immediate),
      0xe2 => Instruction(2, Opcode::SKB, Addressing::Immediate),

      // unofficial_opcodesが必要らしい
      /// RMW 命令
      // DCP
      0xc3 => Instruction(8, Opcode::DCP, Addressing::IndirectX),
      0xc7 => Instruction(5, Opcode::DCP, Addressing::Zeropage),
      0xcf => Instruction(6, Opcode::DCP, Addressing::Absolute),
      0xd3 => Instruction(8, Opcode::DCP, Addressing::IndirectY),
      0xd7 => Instruction(6, Opcode::DCP, Addressing::ZeropageX),
      0xdb => Instruction(7, Opcode::DCP, Addressing::AbsoluteY),
      0xdf => Instruction(7, Opcode::DCP, Addressing::AbsoluteX),
      // ISC
      0xe3 => Instruction(8, Opcode::ISC, Addressing::IndirectX),
      0xe7 => Instruction(5, Opcode::ISC, Addressing::Zeropage),
      0xef => Instruction(6, Opcode::ISC, Addressing::Absolute),
      0xf3 => Instruction(8, Opcode::ISC, Addressing::IndirectY),
      0xf7 => Instruction(6, Opcode::ISC, Addressing::ZeropageX),
      0xfb => Instruction(7, Opcode::ISC, Addressing::AbsoluteX),
      0xff => Instruction(7, Opcode::ISC, Addressing::AbsoluteX),
      // RLA
      0x23 => Instruction(8, Opcode::RLA, Addressing::IndirectX),
      0x27 => Instruction(5, Opcode::RLA, Addressing::Zeropage),
      0x2f => Instruction(6, Opcode::RLA, Addressing::Absolute),
      0x33 => Instruction(8, Opcode::RLA, Addressing::IndirectY),
      0x37 => Instruction(6, Opcode::RLA, Addressing::ZeropageX),
      0x3b => Instruction(7, Opcode::RLA, Addressing::AbsoluteY),
      0x3f => Instruction(7, Opcode::RLA, Addressing::AbsoluteX),
      // RRA
      0x63 => Instruction(8, Opcode::RRA, Addressing::IndirectX),
      0x67 => Instruction(5, Opcode::RRA, Addressing::Zeropage),
      0x6f => Instruction(6, Opcode::RRA, Addressing::Absolute),
      0x73 => Instruction(8, Opcode::RRA, Addressing::IndirectY),
      0x77 => Instruction(6, Opcode::RRA, Addressing::ZeropageX),
      0x7b => Instruction(7, Opcode::RRA, Addressing::AbsoluteY),
      0x7f => Instruction(7, Opcode::RRA, Addressing::AbsoluteX),
      // SLO
      0x03 => Instruction(8, Opcode::SLO, Addressing::IndirectX),
      0x07 => Instruction(5, Opcode::SLO, Addressing::Zeropage),
      0x0f => Instruction(6, Opcode::SLO, Addressing::Absolute),
      0x13 => Instruction(8, Opcode::SLO, Addressing::IndirectY),
      0x17 => Instruction(6, Opcode::SLO, Addressing::ZeropageX),
      0x1b => Instruction(7, Opcode::SLO, Addressing::AbsoluteY),
      0x1f => Instruction(7, Opcode::SLO, Addressing::AbsoluteX),
      // SRE
      0x43 => Instruction(8, Opcode::SRE, Addressing::IndirectX),
      0x47 => Instruction(5, Opcode::SRE, Addressing::Zeropage),
      0x4f => Instruction(6, Opcode::SRE, Addressing::Absolute),
      0x53 => Instruction(8, Opcode::SRE, Addressing::IndirectY),
      0x57 => Instruction(6, Opcode::SRE, Addressing::ZeropageX),
      0x5b => Instruction(7, Opcode::SRE, Addressing::AbsoluteY),
      0x5f => Instruction(7, Opcode::SRE, Addressing::AbsoluteX),

      /// 結合演算子
      0x4b => Instruction(2, Opcode::ALR, Addressing::Immediate),
      0x0b => Instruction(2, Opcode::ANC, Addressing::Immediate),
      0x6b => Instruction(2, Opcode::ARR, Addressing::Immediate),
      0xcb => Instruction(2, Opcode::AXS, Addressing::Immediate),
      // LAX
      0xa3 => Instruction(6, Opcode::LAX, Addressing::IndirectX),
      0xa7 => Instruction(3, Opcode::LAX, Addressing::Zeropage),
      0xaf => Instruction(4, Opcode::LAX, Addressing::Absolute),
      0xb3 => Instruction(5, Opcode::LAX, Addressing::IndirectY),
      0xb7 => Instruction(4, Opcode::LAX, Addressing::ZeropageY),
      0xbf => Instruction(4, Opcode::LAX, Addressing::AbsoluteY),
      // SAX
      0x83 => Instruction(6, Opcode::SAX, Addressing::IndirectX),
      0x87 => Instruction(3, Opcode::SAX, Addressing::Zeropage),
      0x8f => Instruction(4, Opcode::SAX, Addressing::Absolute),
      0x97 => Instruction(4, Opcode::SAX, Addressing::ZeropageY),
      // IGN
      0x0c => Instruction(4, Opcode::IGN, Addressing::Absolute),

      // 4 or 5 cycle と書いてあったけど4っぽい?
      0x1c => Instruction(4, Opcode::IGN, Addressing::AbsoluteX),
      0x3c => Instruction(4, Opcode::IGN, Addressing::AbsoluteX),
      0x5c => Instruction(4, Opcode::IGN, Addressing::AbsoluteX),
      0x7c => Instruction(4, Opcode::IGN, Addressing::AbsoluteX),
      0xdc => Instruction(4, Opcode::IGN, Addressing::AbsoluteX),
      0xfc => Instruction(4, Opcode::IGN, Addressing::AbsoluteX),

      0x04 => Instruction(3, Opcode::IGN, Addressing::Zeropage),
      0x44 => Instruction(3, Opcode::IGN, Addressing::Zeropage),
      0x64 => Instruction(3, Opcode::IGN, Addressing::Zeropage),

      0x14 => Instruction(4, Opcode::IGN, Addressing::ZeropageX),
      0x34 => Instruction(4, Opcode::IGN, Addressing::ZeropageX),
      0x54 => Instruction(4, Opcode::IGN, Addressing::ZeropageX),
      0x74 => Instruction(4, Opcode::IGN, Addressing::ZeropageX),
      0xd4 => Instruction(4, Opcode::IGN, Addressing::ZeropageX),
      0xf4 => Instruction(4, Opcode::IGN, Addressing::ZeropageX),

      // Unknown code となったものをNOPとしておく
      0x02 => Instruction(1, Opcode::NOP, Addressing::Implied),
      0x12 => Instruction(1, Opcode::NOP, Addressing::Implied),
      0x22 => Instruction(1, Opcode::NOP, Addressing::Implied),
      0x32 => Instruction(1, Opcode::NOP, Addressing::Implied),
      0x42 => Instruction(1, Opcode::NOP, Addressing::Implied),
      0x52 => Instruction(1, Opcode::NOP, Addressing::Implied),
      0x62 => Instruction(1, Opcode::NOP, Addressing::Implied),
      0x72 => Instruction(1, Opcode::NOP, Addressing::Implied),
      0x92 => Instruction(1, Opcode::NOP, Addressing::Implied),

      // TODO: たまに未知の値を検出するとパニックするのでNOPで対応
      // TODO: 実装方法が正しいのかは不明 [要検証]
      // _ => panic!(format!("Unknown code 0x{:>02x}", code)),
      _ => {
        let text = format!("Unknown code 0x{:>02x}", code);

        // 本来はすべてpanicさせるべき(?)
        if false { panic!(text); }
        // println!("{}", text);
        Instruction(1, Opcode::NOP, Addressing::Implied)
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

  // フラグ(n-bit目)の読み出し
  fn read_c_flag(&self) -> u8 {
    self.p & (1 << 0)
  }

  fn read_z_flag(&self) -> u8 {
    self.p & (1 << 1)
  }

  fn read_i_flag(&self) -> u8 {
    self.p & (1 << 2)
  }

  fn read_d_flag(&self) -> u8 {
    self.p & (1 << 3)
  }

  fn read_b_flag(&self) -> u8 {
    self.p & (1 << 4)
  }

  fn read_v_flag(&self) -> u8 {
    self.p & (1 << 6)
  }

  fn read_n_flag(&self) -> u8 {
    self.p & (1 << 7)
  }

  // フラグ操作
  fn set_c_flag(&mut self, stat: bool) {
    self.p = if stat {
      self.p | 0x01  // 1 << 0
    } else {
      self.p & (!0x01)
    };
  }

  fn set_z_flag(&mut self, stat: bool) {
    self.p = if stat {
      self.p | 0x02  // 1 << 1
    } else {
      self.p & (!0x02)
    };
  }

  fn set_i_flag(&mut self, stat: bool) {
    self.p = if stat {
      self.p | 0x04  // 1 << 2
    } else {
      self.p & (!0x04)
    };
  }

  fn set_d_flag(&mut self, stat: bool) {
    self.p = if stat {
      self.p | 0x08  // 1 << 3
    } else {
      self.p & (!0x08)
    };
  }

  fn set_b_flag(&mut self, stat: bool) {
    self.p = if stat {
      self.p | 0x10  // 1 << 4
    } else {
      self.p & (!0x10)
    };
  }

  // 1 << 5 は常に1

  fn set_v_flag(&mut self, stat: bool) {
    self.p = if stat {
      self.p | 0x40  // 1 << 6
    } else {
      self.p & (!0x40)
    };
  }

  fn set_n_flag(&mut self, stat: bool) {
    self.p = if stat {
      self.p | 0x80  // 1 << 7
    } else {
      self.p & (!0x80)
    };
  }

  fn fetch_data(
    self,
    addr_mode: Addressing,
    machine: &mut machine::Machine
  ) -> u8 {
    // TODO: 正しいアドレス指定を実装する
    // http://pgate1.at-ninja.jp/NES_on_FPGA/nes_cpu.htm#addressing
    match addr_mode {
      Addressing::Accumulator => self.a,
      _ => 0
    }
  }

  fn fetch_operand(
    self,
    addr_mode: Addressing
  ) -> u16 {
    match addr_mode {
      _ => 0
    }
  }

  // 実行したいニャンね
  pub fn exec(&mut self, machine: &mut machine::Machine) -> u8 {
    // pcからfetchするらしい
    let code = machine.prg_rom[self.pc as usize];
    // let inst = self.convert(code);
    let Instruction(cycle, opcode, addr_mode) = self.convert(code);
    self.pc += 1;

    // println!("0x{:>04x} -> ({:?}, {:?}, {:?})", code, cycle, opcode, addr_mode);
    println!(
      "Exec:
  \x1b[38;2;252;200;0mCycles: {}\x1b[m,
  \x1b[38;2;20;210;240mInstruction: {:?}\x1b[m,
  \x1b[38;2;170;95;230maddr_modeessing: {:?}\x1b[m",
      cycle,
      opcode,
      addr_mode
    );

    // http://obelisk.me.uk/6502/reference.html
    // http://pgate1.at-ninja.jp/NES_on_FPGA/nes_cpu.htm#instruction
    match opcode {
      // 数値演算, 論理演算
      Opcode::ADC => {
        let a = self.a;
        let m = self.fetch_data(addr_mode, machine);
        let c = self.read_c_flag();
        let res = (a + m + c) & 0xff;

        self.set_c_flag((a + m + c) > 0xff);
        self.set_z_flag(res == 0);
        self.set_v_flag((a ^ res) & (m ^ res) & (1 << 7) == 1 << 7);
        self.set_n_flag((res & (1 << 7)) == 1 << 7);

        self.a = res;
      },

      Opcode::SBC => {
        let a = self.a;
        let m = self.fetch_data(addr_mode, machine);
        let c = self.read_c_flag();
        let res = a - m - !c;

        self.set_n_flag((res & (1 << 7)) == 1 << 7);
        self.set_v_flag((a ^ m) & (m ^ res) & (1 << 7) == 1 << 7);
        self.set_z_flag(res == 0);
        self.set_c_flag(res >= 0);
        self.a = res;
      },

      Opcode::AND => {
        let a = self.a;
        let m = self.fetch_data(addr_mode, machine);
        let res = a & m;

        self.set_z_flag(a == 0);
        self.set_n_flag((res & (1 << 7)) == 1 << 7);
        self.a = res;
      },

      Opcode::ORA => {
        let a = self.a;
        let m = self.fetch_data(addr_mode, machine);
        let res = a | m;

        self.set_z_flag(a == 0);
        self.set_n_flag((res & (1 << 7)) == 1 << 7);
        self.a = res;
      },

      Opcode::EOR => {
        let a = self.a;
        let m = self.fetch_data(addr_mode, machine);
        let res = a ^ m;

        self.set_z_flag(a == 0);
        self.set_n_flag((res & (1 << 7)) == 1 << 7);
        self.a = res;
      },

      // bitシフト, bitローテーション
      Opcode::ASL => {
        // モード別でアドレスをFetchしてくる
        let a = self.a;
        let res = a << 1;

        self.set_n_flag((res >> 1) & 1 == 1);
        self.set_z_flag(res == 0);
        self.set_c_flag((a >> 1) & 1 == 1);

        if addr_mode == Addressing::Accumulator {
          self.a = res;
        } else {
          // TODO: wramを書き換える
          // TODO: Fetchしたアドレスに置き換える
          machine.write(0, res);
        }
      },

      Opcode::LSR => {
        let a = self.a;
        let res = a >> 1;

        self.set_n_flag((res >> 7) & 1 == 1 << 7);
        self.set_z_flag(res == 0);
        self.set_c_flag((a >> 0) & 1 == 1);

        if addr_mode == Addressing::Accumulator {
          self.a = res;
        } else {
          machine.write(0, res);
        }
      },

      Opcode::ROL => {
        let a = self.a;
        let res = a << 1;

        self.set_n_flag((res >> 7) & 1 == 1 << 7);
        self.set_z_flag(res == 0);
        self.set_c_flag((a >> 0) & 1 == 1 << 7);

        if addr_mode == Addressing::Accumulator {
          self.a = res;
        } else {
          machine.write(0, res);
        }
      },

      Opcode::ROR => {
        let a = self.a;
        let res = a >> 1;

        self.set_n_flag((res >> 7) & 1 == 1 << 7);
        self.set_z_flag(res == 0);
        self.set_c_flag((a >> 0) & 1 == 1 << 7);

        if addr_mode == Addressing::Accumulator {
          self.a = res;
        } else {
          machine.write(0, res);
        }
      },

      // 条件分岐
      Opcode::BCC => {
        let addr = self.fetch_operand(addr_mode);
        if self.read_c_flag() == 0 {
          self.pc = addr;
        }
      },

      Opcode::BCS => {
        let addr = self.fetch_operand(addr_mode);
        if self.read_c_flag() == 1 {
          self.pc = addr;
        }
      },

      Opcode::BNE => {
        let addr = self.fetch_operand(addr_mode);
        if self.read_z_flag() == 0 {
          self.pc = addr;
        }
      },


      Opcode::BEQ => {
        let addr = self.fetch_operand(addr_mode);
        if self.read_z_flag() == 1 {
          self.pc = addr;
        }
      },

      Opcode::BVC => {
        let addr = self.fetch_operand(addr_mode);
        if self.read_v_flag() == 0 {
          self.pc = addr;
        }
      },

      Opcode::BVS => {
        let addr = self.fetch_operand(addr_mode);
        if self.read_v_flag() == 1 {
          self.pc = addr;
        }
      },

      Opcode::BPL => {
        let addr = self.fetch_operand(addr_mode);
        if self.read_n_flag() == 0 {
          self.pc = addr;
        }
      },

      Opcode::BMI => {
        let addr = self.fetch_operand(addr_mode);
        if self.read_n_flag() == 1 {
          self.pc = addr;
        }
      },

      // bit検査
      Opcode::BIT => {
        let res = self.fetch_data(addr_mode, machine);

        self.set_n_flag(res & (1 << 7) == 1 << 7);
        self.set_v_flag(res & (1 << 6) == 1 << 6);
        self.set_z_flag(self.a & res == 0);
      },

      // ジャンプ命令
      Opcode::JMP => {
        let addr = self.fetch_operand(addr_mode);
        self.pc = addr;
      },

      Opcode::JSR => {
        let addr = self.fetch_operand(addr_mode);
        self.push_stack(machine, (self.pc >> 8) as u8);
        self.push_stack(machine, (self.pc & 0xff) as u8);
        self.pc = addr;
      },

      Opcode::RTS => {
        let lower = self.pop_stack(machine) as u16;
        let higher = self.pop_stack(machine) as u16;
        let addr = ((higher << 8) | lower) + 1;
        self.pc = addr;
      },

      // 割り込み処理
      Opcode::BRK => {
        self.interrupt(
          machine,
          Interrupt::BRK
        );
      },

      Opcode::RTS => {
        let stat = self.pop_stack(machine);
        let lower = self.pop_stack(machine) as u16;
        let higher = self.pop_stack(machine) as u16;

        self.p = stat;
        self.pc = (higher << 8) | lower;
      },

      // 比較演算
      Opcode::CMP => {
        let m = self.fetch_data(addr_mode, machine);
        let res = self.a - m;

        self.set_n_flag((res >> 7) & 1 == 1);
        self.set_z_flag(res == 0);
        self.set_c_flag(self.a >= m);
      },

      Opcode::CPX => {
        let m = self.fetch_data(addr_mode, machine);
        let res = self.x - m;

        self.set_n_flag((res >> 7) & 1 == 1);
        self.set_z_flag(res == 0);
        self.set_c_flag(self.y >= m);
      },

      Opcode::CPY => {
        let m = self.fetch_data(addr_mode, machine);
        let res = self.y - m;

        self.set_n_flag((res >> 7) & 1 == 1);
        self.set_z_flag(res == 0);
        self.set_c_flag(self.x >= m);
      },

      // ワンアゲ, ワンサゲ
      // https://twitter.com/yuki384love/status/1270365593800081408
      Opcode::INC => {
        let addr = self.fetch_operand(addr_mode);
        let m = self.fetch_data(addr_mode, machine);
        let res = m + 1;
        machine.write(addr as usize, res);

        self.set_z_flag(res == 0);
        self.set_n_flag((res >> 7) & 1 == (1 << 7));
      },

      Opcode::DEC => {
        let addr = self.fetch_operand(addr_mode);
        let m = self.fetch_data(addr_mode, machine);
        let res = m - 1;
        machine.write(addr as usize, res);

        self.set_z_flag(res == 0);
        self.set_n_flag((res >> 7) & 1 == (1 << 7));
      }

      _ => {}
    }

    println!(
      "Stat:
  \x1b[38;2;252;200;0mAccumulator: {}\x1b[m,
  \x1b[38;2;20;210;240mFlag: {:b}\x1b[m",
      self.a,
      self.p
    );

    cycle
  }
}
