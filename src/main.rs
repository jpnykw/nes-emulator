extern crate find_folder;
extern crate freetype;
extern crate piston_window;
extern crate image;

use std::time::{SystemTime};
use piston_window::*;
use image::*;
use std::env;

mod instruction;
mod system;
mod machine;
mod ppu;
mod cpu;

const DEBUG_WIDTH: u32 = 250;
const DEBUG_HEIGHT: u32 = 0; // 100;

const WIDTH: u32 = 256;
const HEIGHT: u32 = 240;
const SIZE: f64 = 2.0;

fn main() {
  // デバッグモード判定用
  let args: Vec<String> = env::args().collect();
  let gui_debug = args.contains(&"gdebug".to_string()) || args.contains(&"gdb".to_string()) || args.contains(&"g".to_string());
  let cui_debug = args.contains(&"cdebug".to_string()) || args.contains(&"cdb".to_string()) || args.contains(&"c".to_string());

  // 初期化する
  let mut machine = machine::Machine::new();
  let mut cpu = cpu::Cpu::new();
  let mut ppu = ppu::Ppu::new();

  // カセット読み込み
  let path = "./roms/helloworld.nes"; // it works!
  // let path = "./roms/SHOOT.nes"; // it works!
  let result = system::load_cassette(path.to_string(), cui_debug);

  let (prg_rom, chr_rom) = match result {
    Ok(rom) => rom,
    Err(_) => panic!("ROM切り離せなかった")
  };

  // machineにROMをセット
  machine.set_roms(prg_rom, chr_rom);

  // 電源が入るとRESETの割込処理が走る
  cpu.interrupt(&mut machine, instruction::Interrupt::RESET);

  // GUI
  let opengl = OpenGL::V3_2;
  let width = WIDTH * SIZE as u32 + if gui_debug { DEBUG_WIDTH * SIZE as u32 } else { 0 };
  let height = HEIGHT * SIZE as u32 + if gui_debug { DEBUG_HEIGHT * SIZE as u32 } else { 0 };
  let mut window: PistonWindow = WindowSettings::new(format!("NES Emulator ({})", path), (width, height))
  .graphics_api(opengl)
  .exit_on_esc(true)
  .build()
  .expect("Failed to build window.");

  // フォントの読み込み
  let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
  let ref font = assets.join("Arimo for Powerline.ttf");

  let factory = window.create_texture_context();
  let mut glyphs = Glyphs::new(font, factory, TextureSettings::new()).unwrap(); // thanks, @megumish

  let mut screen = ImageBuffer::new(
    WIDTH * SIZE as u32,
    HEIGHT * SIZE as u32
  );

  let mut texture_context = TextureContext {
    factory: window.factory.clone(),
    encoder: window.factory.create_command_buffer().into(),
  };

  let mut texture = Texture::from_image(
    &mut texture_context,
    &screen,
    &TextureSettings::new()
  ).expect("Failed to create texture.");

  let mut cycles = 0; // タイミング調整用
  let mut cpu_time: i128 = 0; // 命令の実行回数を計測
  let start_at = SystemTime::now(); // システムの起動時間を計測

  let mut events = Events::new(EventSettings::new());
  while let Some(e) = events.next(&mut window) {
    if let Some(args) = e.render_args() {
      // cpuを進める
      // こういう事で合ってる?
      if cycles == 0 {
        println!("\n!-------------------- {} --------------------!", cpu_time);
        cycles = cpu.exec(&mut machine);
        cpu_time += 1;
      }  else {
        cycles -= 1;
      }

      // 直接描画してみる
      for i in 0 .. (32 * 10) {
        let base = 16 * (0x21 + i); // $21: 記号と数字, $41: 英大文字と感嘆/疑問符
        let pattern_low = &chr_rom[base .. base + 0x8]; // 0 ~ 7
        let pattern_high = &chr_rom[base + 0x8 .. base + 0x10]; // 8 ~ 15

        for y in 0 .. 8 {
          for x in 0 .. 8 {
            fn is_put(v: u8, x: u8) -> bool { (v >> x) & 1 == 1 }
            let dx = ((7 - x) + (i + 1) % 32 * 8) as u32;
            let dy = y as u32 + ((i + 1) / 32) as u32 * 8;

            screen.put_pixel(
              dx, dy,
              if is_put(pattern_low[y], x as u8) { Rgba([255, 255, 255, 127]) }
              else { Rgba([0; 4]) }
            );

            screen.put_pixel(
              dx, dy,
              if is_put(pattern_high[y], x as u8) { Rgba([255, 255, 255, 127]) }
              else { Rgba([0; 4]) }
            );
          }
        }
      }

      texture.update(
        &mut texture_context,
        &screen
      ).unwrap();

      window.draw_2d(&e, |c, g, d| {
        clear([0.0, 0.0, 0.0, 1.0], g);
        texture_context.encoder.flush(d);
        image(&texture, c.transform.scale(SIZE, SIZE), g);

        // デバッグ用の背景を右側に描画する
        if gui_debug {
          rectangle(
            [0.0, 0.1, 0.2, 1.0],
            [
              WIDTH as f64 * SIZE + 1.0,
              0.0,
              DEBUG_WIDTH as f64 * SIZE,
              height as f64,
            ], // x, y, w, h
            c.transform, g
          );

          // レジスタ
          let mut transform = c.transform.trans(WIDTH as f64 * SIZE + 20.0, 30.0);
          let mut text = format!("A: {:<08x} X: {:<08x} Y: {:<08x}", cpu.a, cpu.x, cpu.y);

          text::Text::new_color([1.0, 1.0, 1.0, 1.0], 15).draw(
            &text,
            &mut glyphs,
            &c.draw_state,
            transform,
            g
          ).unwrap();

          // フラグ
          text = format!("PC: {:<08x}, SP: {:<08x}", cpu.pc, cpu.sp);
          transform = c.transform.trans(WIDTH as f64 * SIZE + 20.0, 60.0);

          text::Text::new_color([1.0, 1.0, 1.0, 1.0], 15).draw(
            &text,
            &mut glyphs,
            &c.draw_state,
            transform,
            g
          ).unwrap();

          text = format!("Flags: {:<08b}", cpu.p);
          transform = c.transform.trans(WIDTH as f64 * SIZE + 20.0, 90.0);

          text::Text::new_color([1.0, 1.0, 1.0, 1.0], 15).draw(
            &text,
            &mut glyphs,
            &c.draw_state,
            transform,
            g
          ).unwrap();

          // 命令実行回数、起動時間
          text = format!("Executions count: {:<010}", cpu_time);
          transform = c.transform.trans(WIDTH as f64 * SIZE + 20.0, height as f64 - 60.0);

          text::Text::new_color([1.0, 1.0, 1.0, 1.0], 15).draw(
            &text,
            &mut glyphs,
            &c.draw_state,
            transform,
            g
          ).unwrap();

          text = format!("Startup time: {:<010}(s)",
            match start_at.elapsed() {
              Ok(elapsed) => elapsed.as_secs(),
              Err(_) => panic!()
            }
          );
          transform = c.transform.trans(WIDTH as f64 * SIZE + 20.0, height as f64 - 30.0);

          text::Text::new_color([1.0, 1.0, 1.0, 1.0], 15).draw(
            &text,
            &mut glyphs,
            &c.draw_state,
            transform,
            g
          ).unwrap();

          glyphs.factory.encoder.flush(d);
        }
      });
    }
  }
}

// テストクン
#[test]
fn cpu_register() {
  let module = cpu::Cpu::new();
  assert_eq!(module.a, 0);
  assert_eq!(module.x, 0);
  assert_eq!(module.y, 0);
  assert_eq!(module.pc, 0);
  assert_eq!(module.sp, 0xfe);
  assert_eq!(module.p, 0x20);
}

#[test]
fn load_cassette() {
  let path = "./roms/helloworld.nes".to_string();
  let result = system::load_cassette(path, false);

  match result {
    Ok(_) => (),
    _ => panic!("カスのカセット、カスット") // 了解！
  }
}

#[test]
fn check_rom_data() {
  let prg_data = [0x78, 0xa2, 0xff, 0x9a, 0xa9, 0x00, 0x8d, 0x00, 0x20, 0x8d]; // from 0
  let chr_data = [0x1c, 0x3e, 0x3e, 0x3e, 0x1c, 0x1c, 0x1c, 0x1c, 0x18, 0x3c]; // from 528
  let path = "./roms/helloworld.nes".to_string();
  let result = system::load_cassette(path, false);

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

#[test]
fn stack_and_pop() {
  let addr = 0x1fe;
  let mut cpu = cpu::Cpu::new();
  let mut machine = machine::Machine::new();

  assert_eq!(cpu.pop_stack(&mut machine), 0); // sp がインクリメントされる
  cpu.push_stack(&mut machine, 0x7b);
  assert_eq!(cpu.pop_stack(&mut machine), 0x7b); // インクリメントされた分ずらす
}
