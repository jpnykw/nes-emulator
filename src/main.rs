mod system;
mod cpu;

fn main() {
  let path = "./rom/helloworld.nes".to_string();
  system::header_process(path);
}
