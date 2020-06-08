const WRAM_SIZE: usize = 0x0800; // 2KiB
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
  pub nametable: [[u8; NAME_TABLE_SIZE]; 4]
}

impl Machine {
  pub fn new() -> Self {
    Self {
      wram: [0; WRAM_SIZE],
      prg_rom: [0; PRG_ROM_SIZE],
      chr_rom: [0; CHR_ROM_SIZE],
      nametable: [[0; NAME_TABLE_SIZE]; 4]
    }
  }

  pub fn set_roms(
    &mut self,
    prg_rom: [u8; PRG_ROM_SIZE],
    chr_rom: [u8; CHR_ROM_SIZE]
  ) {
    self.prg_rom = prg_rom;
    self.chr_rom = chr_rom;
  }

  pub fn store(&mut self, addr: usize, val: u8) {
    self.wram[addr] = val;
  }

  pub fn fetch(self, addr: usize) -> u8 {
    self.wram[addr]
  }

  pub fn write(&mut self, addr: usize, val: u8) {
    // TODO: PPUのミラーなどを考慮する
    self.wram[addr] = val;
  }
}
