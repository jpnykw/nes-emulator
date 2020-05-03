mod instruction;
mod system;
mod machine;
mod ppu;
mod cpu;

fn main() {
  let path = "./roms/helloworld.nes".to_string();
  let result = system::header_process(path);

  let mut ppu = ppu::Ppu::new();
  let mut cpu = cpu::Cpu::new();
  let machine = machine::Machine::new();

  // 電源が入るとRESETの割込処理が走る
  cpu.interrupt(instruction::Interrupt::RESET);
}

// テストクン
#[test]
fn cpu_register() {
  let module = cpu::Cpu::new();
  assert_eq!(module.a, 0);
  assert_eq!(module.x, 0);
  assert_eq!(module.y, 0);
  assert_eq!(module.pc, 0);
  assert_eq!(module.sp, 0);
  assert_eq!(module.p, 0);
}

#[test]
fn load_cassette() {
  let path = "./roms/helloworld.nes".to_string();
  let result = system::header_process(path);

  match result {
    Ok(_) => (),
    _ => panic!("お前のカセットぶっ壊れとるよwwwww") // 了解！
  }
}

#[test]
fn check_rom_data() {
  let prg_data = [0x78, 0xa2, 0xff, 0x9a, 0xa9, 0x00, 0x8d, 0x00, 0x20, 0x8d]; // from 0
  let chr_data = [0x1c, 0x3e, 0x3e, 0x3e, 0x1c, 0x1c, 0x1c, 0x1c, 0x18, 0x3c]; // from 528
  let path = "./roms/helloworld.nes".to_string();
  let result = system::header_process(path);

  for id in 0 .. 10 {
    match &result {
      Ok((prg_rom, chr_rom)) => {
        assert_eq!(prg_rom[id], prg_data[id]);
        assert_eq!(chr_rom[id + 528], chr_data[id]);
      },

      _ => {}
    }
  }
}
