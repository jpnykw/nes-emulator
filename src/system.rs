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

pub fn debug(val: u8, eol: bool) {
  print!("{:>04x}{}", val, if eol { "\n" } else { " " });
}

pub fn load_cassette(path: String) -> Result<(Vec<u8>, Vec<u8>), String> {
  let mut prg_rom = [0; 0x8000];
  let mut chr_rom = [0; 0x2000];
  let ines = read_nes(path);

  match ines {
    Ines::Data(file, buffer) => {
      // "NES" Header
      if buffer[0] != 0x4e { return Err("expect 0x4e".to_string()); }
      if buffer[1] != 0x45 { return Err("expect 0x45".to_string()); }
      if buffer[2] != 0x53 { return Err("expect 0x53".to_string()); }
      if buffer[3] != 0x1a { return Err("expect 0x1a".to_string()); }

      let prg_banks = buffer[4] as usize;
      let chr_banks = buffer[5] as usize;
      let prg_bytes = prg_banks * 0x4000;
      let chr_bytes = chr_banks * 0x2000;

      let header = 16;
      let prg_addr = header;
      let chr_addr = prg_addr + prg_bytes;
      println!("PRG-ROM bytes: \x1b[38;5;51m0x{:>08x}\x1b[m", prg_bytes);
      println!("CHR-ROM bytes: \x1b[38;5;51m0x{:>08x}\x1b[m", chr_bytes);
      println!("PRG-ROM mapping: \x1b[38;5;51m0x{:>08x} ~ 0x{:>08x}\x1b[m", prg_addr, prg_addr + prg_bytes);
      println!("CHR-ROM mapping: \x1b[38;5;51m0x{:>08x} ~ 0x{:>08x}\x1b[m", chr_addr, chr_addr + chr_bytes);

      if false {
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
      }

      if false {
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

      if false {
        let mut cpu = cpu::Cpu::new();
        for addr in prg_addr .. prg_addr + prg_bytes {
          cpu.convert(buffer[addr]);
        }
      }

      let mut cpu = cpu::Cpu::new();
      let mode = false; // ROMの中身の表示/非表示を切換え

      if mode { println!("\n========== PRG-ROM =========="); }
      for addr in 0 .. prg_bytes {
        prg_rom[addr] = buffer[prg_addr + addr];
        if mode { debug(prg_rom[addr], (addr + 1) % 17 == 0); }
      }

      if mode { println!("\n========== CHR-ROM =========="); }
      for addr in 0 .. chr_bytes {
        chr_rom[addr] = buffer[chr_addr + addr];
        if mode { debug(chr_rom[addr], (addr + 1) % 17 == 0); }
      }

      println!();
    },

    _ => panic!("Invalid file type")
  }

  Ok((
    prg_rom.to_vec(),
    chr_rom.to_vec()
  ))
}
