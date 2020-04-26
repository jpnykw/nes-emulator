mod instruction;
mod cpu;

#[test]
fn cpu_test() {
  let module = cpu::Cpu::new();
  assert_eq!(module.a, 0);
  assert_eq!(module.x, 0);
  assert_eq!(module.y, 0);
  assert_eq!(module.pc, 0);
  assert_eq!(module.sp, 0);
  assert_eq!(module.p, 0);
}

