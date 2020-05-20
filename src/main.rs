/*
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::{WindowSettings};
*/

extern crate find_folder;
extern crate freetype;
extern crate piston_window;
extern crate image;

use piston_window::*;
use image::*;
use std::env;

mod instruction;
mod system;
mod machine;
mod ppu;
mod cpu;

const DEBUG_WIDTH: u32 = 200;
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
  let path = "./roms/helloworld.nes".to_string();
  let result = system::load_cassette(path, cui_debug);

  let (prg_rom, chr_rom) = match result {
    Ok(rom) => rom,
    Err(_) => panic!("ROM切り離せなかった")
  };

  // machineにROMをセット
  machine.set_roms(prg_rom, chr_rom);

  // 直接CPUを実行していく(実際はループ)
  for _ in 0 .. 5 {
    cpu.exec(&mut machine);
  }

  // 電源が入るとRESETの割込処理が走る
  cpu.interrupt(&mut machine, instruction::Interrupt::RESET);

  // GUI
  let opengl = OpenGL::V3_2;
  let mut window: PistonWindow = WindowSettings::new(
    "NES Emulator",
    (WIDTH * SIZE as u32 + if gui_debug { DEBUG_WIDTH * SIZE as u32 } else { 0 },
    HEIGHT * SIZE as u32)
  )
  .graphics_api(opengl)
  .exit_on_esc(true)
  .build()
  .expect("Failed to build window.");

  // フォントの読み込み
  let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
  let ref font = assets.join("Geomanist-Regular.otf");
  let factory = window.factory.clone();
  // let mut glyphs = Glyphs::new(font, factory, TextureSettings::new()).unwrap(); // 謎多き人物

  let mut screen = ImageBuffer::new(
    WIDTH * SIZE as u32,
    HEIGHT * SIZE as u32
  );

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
          if is_put(pattern_low[y], x as u8) { Rgba([255, 255, 255, 50]) }
          else { Rgba([0; 4]) }
        );

        screen.put_pixel(
          dx, dy,
          if is_put(pattern_high[y], x as u8) { Rgba([255, 255, 255, 50]) }
          else { Rgba([0; 4]) }
        );
      }
    }
  }

  let mut texture_context = TextureContext {
    factory: window.factory.clone(),
    encoder: window.factory.create_command_buffer().into(),
  };

  let mut texture = Texture::from_image(
    &mut texture_context,
    &screen,
    &TextureSettings::new()
  ).expect("Failed to create texture.");

  let mut events = Events::new(EventSettings::new());
  while let Some(e) = events.next(&mut window) {
    if let Some(args) = e.render_args() {
      window.draw_2d(&e, |c, g, _| {
        clear([0.0, 0.0, 0.0, 1.0], g);
        image(&texture, c.transform.scale(SIZE, SIZE), g);

        // デバッグ用の背景を右側に描画する
        if gui_debug {
          rectangle(
            [0.0, 0.1, 0.2, 1.0],
            [
              WIDTH as f64 * SIZE + 1.0,
              0.0,
              DEBUG_WIDTH as f64 * SIZE,
              HEIGHT as f64 * SIZE,
            ], // x, y, w, h
            c.transform, g
          );

          // デバッグ用に情報を描画する
          /*
          let transform = c.transform.trans(100.0, 100.0);
          text::Text::new_color([0.0, 0.0, 0.0, 1.0], 32).draw(
            "unchi",
            &mut glyphs,
            &c.draw_state,
            transform,
            g
          );
          */
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

  assert_eq!(cpu.pop_stack(&mut machine, addr), 0); // sp がインクリメントされる
  cpu.push_stack(&mut machine, 0x7b);
  assert_eq!(cpu.pop_stack(&mut machine, addr + 1), 0x7b); // インクリメントされた分ずらす
}
