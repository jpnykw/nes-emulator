mod instruction;
mod system;
mod machine;
mod cpu;

fn main() {
  let path = "./rom/helloworld.nes".to_string();
  let result = system::header_process(path);

  let mut cpu = cpu::Cpu::new();
  let machine = machine::Machine::new();

  // 電源が入るとRESETの割込処理が走る
  cpu.interrupt(instruction::Interrupt::RESET);
}
