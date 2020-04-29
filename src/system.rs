use std::fs::File;
use std::io::Read;
use super::cpu;

pub enum Ines {
  Data(
    usize,
    Vec<u8>
  )
}

pub fn read_nes(path: String) -> Ines {
  println!("Target: {}", path);
  let mut file = File::open(path).unwrap();
  let mut buffer: Vec<u8> = Vec::new();

  Ines::Data(
    file.read_to_end(&mut buffer).unwrap(),
    buffer
  )
}

pub fn header_process(path: String) -> Result<(), String> {
  let ines = read_nes(path);

  match ines {
    Ines::Data(file, buffer) => {
      // Constant $4E $45 $53 $1A
      if buffer[0] != 0x4e { return Err("expect 0x4e".to_string()); }
      if buffer[1] != 0x45 { return Err("expect 0x45".to_string()); }
      if buffer[2] != 0x53 { return Err("expect 0x53".to_string()); }
      if buffer[3] != 0x1a { return Err("expect 0x1a".to_string()); }

      let prg_block = buffer[4] as usize;
      let chr_block = buffer[5] as usize;
      let prg_bytes = prg_block * 0x4000;
      let chr_bytes = chr_block * 0x2000;

      let header = 16;
      let prg_addr = header;
      let chr_addr = header + prg_addr + prg_bytes;
      println!("PRG-ROM bytes: \x1b[38;5;51m0x{:>08x} ~ 0x{:>08x}\x1b[m", prg_addr, prg_addr + prg_bytes);
      println!("CHR-ROM bytes: \x1b[38;5;51m0x{:>08x} ~ 0x{:>08x}\x1b[m", chr_addr, chr_addr + chr_bytes);

      // TODO: mainに移動させる
      println!("Data of PRG-ROM");
      for i in prg_addr .. prg_addr + 30 {
        let mut line = format!("{:<03x}", i);
        for j in 0 .. 9 {
          line = format!(
            "{} \x1b[38;5;69m{:>08x}\x1b[m", line,
            buffer[i + j]
          );
        }
        println!("{}", line);
      }
      println!("...");

      {
        let mut cpu = cpu::Cpu::new();
        println!("Converted data of PRG-ROM");
        for i in prg_addr .. prg_addr + 30 {
          let mut line = format!("{:<03x}", i);
          for j in 0 .. 3 {
            line = format!(
              "{} \x1b[38;5;69m{:?}\x1b[m", line,
              cpu.convert(buffer[i + j])
            );
          }
          println!("{}", line);
        }
        println!("...");
      }
    },

    _ => panic!("Invalid file type")
  }

  println!("OK");

  // TODO: バッファを返却する
  Ok(())
}
