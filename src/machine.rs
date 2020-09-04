const WRAM_SIZE: usize = 0x800; // 2KiB
const PRG_ROM_SIZE: usize = 0x8000;
const CHR_ROM_SIZE: usize = 0x2000;

// 0 = $2000, 1 = $2400, 2 = $2800, 3 = $2c00
const NAME_TABLE_SIZE: usize = 0x400;

#[derive(Copy, Clone)]
pub struct Machine {
  pub wram: [u8; WRAM_SIZE],

  pub prg_rom: [u8; PRG_ROM_SIZE],
  pub chr_rom: [u8; CHR_ROM_SIZE],

  // for PPU
  pub nametable: [[u8; NAME_TABLE_SIZE]; 4],
  pub ppu_register: [u8; 8],

  pub prg_bytes: usize,
  pub chr_bytes: usize
}

impl Machine {
  pub fn new() -> Self {
    Self {
      wram: [0; WRAM_SIZE],
      prg_rom: [0; PRG_ROM_SIZE],
      chr_rom: [0; CHR_ROM_SIZE],

      nametable: [[0; NAME_TABLE_SIZE]; 4],
      ppu_register: [0; 8],

      prg_bytes: 0,
      chr_bytes: 0
    }
  }

  pub fn set_roms(&mut self, prg_rom: [u8; PRG_ROM_SIZE], chr_rom: [u8; CHR_ROM_SIZE]) {
    self.prg_rom = prg_rom;
    self.chr_rom = chr_rom;
  }

  pub fn write(&mut self, addr: usize, val: u8) {
    if addr < 0x2000 {
      self.wram[addr] = val;
    } else {
      // println!("ppu reg -> {}", (addr - 0x2000) % 8);
      println!("ppu addr ${:<04x}", addr - 0x2000);

      // メモリマップ上$2000~に配置
      let ppu_addr = addr - 0x2000;
      match ppu_addr {
        7 => {
          self.ppu_register[7] = val
          // Hoge
        },

        _ => {
          self.ppu_register[ppu_addr] = val
        }
      }
    }
  }

  pub fn read(&mut self, addr: usize) -> u8 {
    if addr >= self.prg_bytes {
        self.prg_rom[addr - self.prg_bytes]
    } else {
        self.prg_rom[addr]
    }
  }
}
