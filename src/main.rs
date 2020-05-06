/*
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::{WindowSettings};
*/

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
const SIZE: f64 = 1.0;

fn main() {
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
      (WIDTH * SIZE as u32,
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
  for i in 0 .. 25 {
    let base = 16 * (33 + i);
    let sprite_under = &chr_rom[base .. base + 0x8]; // 0 ~ 7
    let sprite_over = &chr_rom[base + 0x8 .. base + 0x10]; // 8 ~ 15

    for y in 0 .. 8 {
      let line = format!("{:>08b}", sprite_under[y]);
      for x in 0 .. 8 {
        screen.put_pixel(
          (x + i * 8) as u32, y as u32,
          if line.chars().nth(x) == Some('1') { Rgba([255, 255, 255, 50]) }
          else { Rgba([0; 4]) }
        );
      }
    }

    for y in 0 .. 8 {
      let line = format!("{:>08b}", sprite_over[y]);
      for x in 0 .. 8 {
        screen.put_pixel(
          (x + i * 8) as u32, y as u32,
          if line.chars().nth(x) == Some('1') { Rgba([255, 255, 255, 50]) }
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
