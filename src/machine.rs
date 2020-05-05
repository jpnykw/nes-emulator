const WRAM_SIZE: usize = 0x0800; // 2KiB

// 0 = $2000, 1 = $2400, 2 = $2800, 3 = $2c00
const NAME_TABLE_SIZE: usize = 0x400;

#[derive(Copy, Clone)]
pub struct Machine {
  pub wram: [u8; WRAM_SIZE],

  // for PPU
  pub nametable: [[u8; NAME_TABLE_SIZE]; 4]
}

impl Machine {
  pub fn new() -> Self {
    Self {
      wram: [0; WRAM_SIZE],
      nametable: [[0; NAME_TABLE_SIZE]; 4]
    }
  }

  pub fn store(&mut self, addr: usize, val: u8) {
    self.wram[addr] = val;
  }

  pub fn fetch(self, addr: usize) -> u8 {
    self.wram[addr]
  }
}
