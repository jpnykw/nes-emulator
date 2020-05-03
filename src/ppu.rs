const VRAM_SIZE: usize = 0x0800;

pub struct Ppu {
  pub vram: [u8; VRAM_SIZE]
}

impl Ppu {
  pub fn new() -> Self {
    Self {
      vram: [0; VRAM_SIZE]
    }
  }

  pub fn fetch(chr_rom: Vec<u8>) -> u8 {
    0
  }
}
