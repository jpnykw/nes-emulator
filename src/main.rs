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

mod instruction;
mod system;
mod machine;
mod ppu;
mod cpu;

const WIDTH: u32 = 256;
const HEIGHT: u32 = 240;
const SIZE: f64 = 2.0;

fn main() {
  // デバッグ情報の描画用にフォントを読み込む
  let assets = find_folder::Search::ParentsThenKids(3, 3)
    .for_folder("assets")
    .unwrap();

  let frtp = freetype::Library::init().unwrap();
  let font = assets.join("Geomanist-Regular.otf");
  let mut face = frtp.new_face(&font, 0).unwrap();
  face.set_pixel_sizes(0, 30).unwrap();

  let path = "./roms/helloworld.nes".to_string();
  let result = system::load_cassette(path);

  let chr_rom = match result {
    Ok(rom) => rom.1,
    Err(_) => Vec::new()
  };

  // let base = 0x210; // 16 の倍数になっているので良さそう💃
  // for i in 0 .. 8 {
  //   println!("{:>08b}", sprite_under[i]);
  // }

  let mut machine = machine::Machine::new();
  let mut cpu = cpu::Cpu::new();
  let mut ppu = ppu::Ppu::new();

  // 電源が入るとRESETの割込処理が走る
  cpu.interrupt(&mut machine, instruction::Interrupt::RESET);

  // GUI
  let opengl = OpenGL::V3_2;
  let mut window: PistonWindow = WindowSettings::new(
      "NES Emulator",
      (WIDTH * SIZE as u32 + 200 * SIZE as u32,
      HEIGHT * SIZE as u32)
    )
    .graphics_api(opengl)
    .exit_on_esc(true)
    .build()
    .expect("Failed to build window.");

  let mut screen = ImageBuffer::new(
    WIDTH * SIZE as u32,
    HEIGHT * SIZE as u32
  );

  // 描画してみる
  for i in 0 .. 30 {
    let base = 16 * (65 + i); // 基準となるアドレス 33: Symbol, 0-9, 65: A~Z, !?
    let sprite_under = &chr_rom[base .. base + 0x8]; // 0 ~ 7
    let sprite_over = &chr_rom[base + 0x8 .. base + 0x10]; // 8 ~ 15

    for y in 0 .. 8 {
      for x in 0 .. 8 {
        screen.put_pixel(
          ((8 - x) + i * 8) as u32, y as u32,
          if (sprite_under[y] >> x) & 1 == 1 { Rgba([255, 255, 255, 50]) }
          else { Rgba([0; 4]) }
        );

        screen.put_pixel(
          ((8 - x) + i * 8) as u32, y as u32,
          if (sprite_over[y] >> x) & 1 == 1 { Rgba([255, 255, 255, 50]) }
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

        rectangle(
          [0.0, 0.1, 0.2, 1.0],
          [
            WIDTH as f64 * SIZE + 1.0,
            0.0,
            200.0 * SIZE,
            HEIGHT as f64 * SIZE,
          ], // x, y, w, h
          c.transform, g
        );
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
  let result = system::load_cassette(path);

  match result {
    Ok(_) => (),
    _ => panic!("お前のカセット、まるでうんこだね") // 了解！
  }
}

#[test]
fn check_rom_data() {
  let prg_data = [0x78, 0xa2, 0xff, 0x9a, 0xa9, 0x00, 0x8d, 0x00, 0x20, 0x8d]; // from 0
  let chr_data = [0x1c, 0x3e, 0x3e, 0x3e, 0x1c, 0x1c, 0x1c, 0x1c, 0x18, 0x3c]; // from 528
  let path = "./roms/helloworld.nes".to_string();
  let result = system::load_cassette(path);

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
