use super::machine;

#[derive(Debug, Copy, Clone)]
pub struct Ppu {
}

impl Ppu {
  pub fn new() -> Self {
    Self {
    }
  }

  pub fn exec(machine: machine::Machine) {
    // レジスタの内容をもとに色の情報などを渡す
  }
}
