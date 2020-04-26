const WRAM_SIZE: usize = 0x0800; // 2KiB

pub struct Machine {
  pub wram: [u8; WRAM_SIZE]
}

impl Machine {
  pub fn new() -> Self {
    Self {
      wram: [0; WRAM_SIZE]
    }
  }
}
