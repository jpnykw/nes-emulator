const WRAM_SIZE: usize = 0x0800; // 2KiB

#[derive(Copy, Clone)]
pub struct Machine {
  pub wram: [u8; WRAM_SIZE]
}

impl Machine {
  pub fn new() -> Self {
    Self {
      wram: [0; WRAM_SIZE]
    }
  }

  pub fn store(&mut self, addr: usize, val: u8) {
    self.wram[addr] = val;
  }

  pub fn fetch(self, addr: usize) -> u8 {
    self.wram[addr]
  }
}
