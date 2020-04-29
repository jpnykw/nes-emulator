mod instruction;
mod system;
mod machine;
mod cpu;

fn main() {
  // カセットを読み込む
  let path = "./roms/helloworld.nes".to_string();
  // let path = "./roms/SHOOT.nes".to_string();
  let result = system::header_process(path);

  let mut cpu = cpu::Cpu::new();
  let machine = machine::Machine::new();

  // 電源が入るとRESETの割込処理が走る
  cpu.interrupt(instruction::Interrupt::RESET);
  // println!("{:?}", cpu.convert(0xa9));
}
