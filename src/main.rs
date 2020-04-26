mod instruction;
mod system;
mod machine;
mod cpu;

fn main() {
  let path = "./rom/helloworld.nes".to_string();
  let result = system::header_process(path);

  let mut cpu = cpu::Cpu::new();
  let machine = machine::Machine::new();

  cpu.interrupt(instruction::Interrupt::RESET);
  cpu.interrupt(instruction::Interrupt::RESET);
}
