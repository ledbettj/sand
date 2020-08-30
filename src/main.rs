use std::time::Instant;

use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

mod dot;
mod sandbox;

use sandbox::Sandbox;

const WIDTH : u32 = 400;
const HEIGHT : u32 = 300;

fn main() -> Result<(), Error> {
  let event_loop = EventLoop::new();
  let mut sandbox = Sandbox::new(WIDTH as usize, HEIGHT as usize);
  let mut input = WinitInputHelper::new();
  let mut draw = dot::Dot::Sand { temp: 0 };

  let window = {
    let size = LogicalSize::new(WIDTH * 3, HEIGHT * 3);
    WindowBuilder::new()
      .with_title("Sand")
      .with_inner_size(size)
      .with_min_inner_size(size)
      .build(&event_loop)
      .expect("Failed to create window")
  };

  let mut pixels = {
    let size = window.inner_size();
    let texture = SurfaceTexture::new(size.width, size.height, &window);
    Pixels::new(WIDTH, HEIGHT, texture)?
  };

  let mut paused = false;
  let mut time = Instant::now();
  event_loop.run(move |event, _, cf| {
    if let Event::RedrawRequested(_) = &event {
      let frame = pixels.get_frame();
      sandbox.draw(frame);
      if pixels.render().is_err() {
        *cf = ControlFlow::Exit;
        return;
      }
    }

    if input.update(&event) {
      if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
        *cf = ControlFlow::Exit;
        return;
      }
      if input.key_pressed(VirtualKeyCode::Space) {
        paused = !paused;
      }

      let (mouse_cell, mouse_prev_cell) = input
        .mouse()
        .map(|(mx, my)| {
          let (dx, dy) = input.mouse_diff();
          let prev_x = mx - dx;
          let prev_y = my - dy;

          let (mx_i, my_i) = pixels
            .window_pos_to_pixel((mx, my))
            .unwrap_or_else(|pos| pixels.clamp_pixel_pos(pos));

          let (px_i, py_i) = pixels
            .window_pos_to_pixel((prev_x, prev_y))
            .unwrap_or_else(|pos| pixels.clamp_pixel_pos(pos));

          (
            (mx_i as isize, my_i as isize),
            (px_i as isize, py_i as isize),
          )
        })
        .unwrap_or_default();

      if input.mouse_held(0) {
        let x0 : isize = mouse_prev_cell.0.max(0).min(WIDTH as isize);
        let y0 : isize = mouse_prev_cell.1.max(0).min(HEIGHT as isize);
        for (x, y) in line_drawing::Bresenham::new((x0, y0), mouse_cell) {
          sandbox.set((x as usize, y as usize), draw);
        }
      } else if input.mouse_pressed(1) {
        draw = draw.next();
      }

      let now = Instant::now();
      let dt = now.duration_since(time);
      time = now;
      if !paused {
        sandbox.step(&dt);
      }
      window.request_redraw();
    }
  });
}
