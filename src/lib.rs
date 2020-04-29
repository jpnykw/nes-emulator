mod instruction;
mod system;
mod cpu;

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
fn nes_header() {
  let path = "./roms/helloworld.nes".to_string();
  let result = system::header_process(path);
  assert_eq!(result, Ok(()));
}
