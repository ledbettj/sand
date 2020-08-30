use std::time::Duration;

use crate::dot::Dot;

use rand::prelude::*;

pub struct Sandbox {
  dots: Vec<Vec<Dot>>,
  rng: ThreadRng,
}

impl Sandbox {
  pub fn new(w: usize, h: usize) -> Sandbox {
    let mut dots = Vec::with_capacity(h);

    for _ in 0..h {
      dots.push((0..w).map(|_| Dot::default()).collect());
    }

    Sandbox { dots, rng: rand::thread_rng() }
  }

  pub fn width(&self) -> usize {
    self.dots[0].len()
  }

  pub fn height(&self) -> usize {
    self.dots.len()
  }

  pub fn set(&mut self, pos: (usize, usize), dot: Dot) {
    self.dots[pos.1][pos.0] = dot;
  }

  pub fn draw(&self, frame: &mut [u8]) {
    let w = self.width();

    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
      let x = i % w;
      let y = i / w;
      self.dots[y][x].draw(pixel);
    }
  }

  fn fall_neighbors(&mut self, pos: (isize, isize)) -> Vec<(isize, isize)> {
    let mut result = Vec::with_capacity(5);

    result.push((pos.0, pos.1 + 1));

    if self.rng.gen() {
      result.push((pos.0 - 1, pos.1 + 1));
      result.push((pos.0 + 1, pos.1 + 1));
    } else {
      result.push((pos.0 + 1, pos.1 + 1));
      result.push((pos.0 - 1, pos.1 + 1));
    }
    result
  }

  fn flow_neighbors(&mut self, pos: (isize, isize)) -> Vec<(isize, isize)> {
    let mut result = Vec::with_capacity(5);

    if self.rng.gen() {
      result.push((pos.0 - 1, pos.1));
      result.push((pos.0 + 1, pos.1));
    } else {
      result.push((pos.0 + 1, pos.1));
      result.push((pos.0 - 1, pos.1));
    }
    result
  }

  fn neighbors(&mut self, pos: (usize, usize)) -> Vec<(usize, usize)> {
    let offsets = [(0, 1), (1, 1), (1, 0), (0, -1), (-1, -1), (-1, 0)];
    let w = self.width() as isize;
    let h = self.height() as isize;
    let mut results = offsets
      .iter()
      .flat_map(|(ox, oy)|{
        let x = pos.0 as isize + ox;
        let y = pos.1 as isize + oy;

        if x < 0 || y < 0 || x >= w || y >= h {
          None
        } else {
          Some((x as usize, y as usize))
        }

      }).collect::<Vec<(usize, usize)>>();

    results.shuffle(&mut self.rng);
    results
  }

  pub fn step(&mut self, dt: &Duration) {
    self.step_attrs(dt);
    self.step_fall(dt);
  }

  pub fn step_attrs(&mut self, _dt: &Duration) {
    for y in 0..self.height() {
      for x in 0..self.width() {
        let dot = self.dots[y][x];

        for (nx, ny) in self.neighbors((x, y)) {
          let n = &mut self.dots[ny][nx];

          match (dot, n) {
            (Dot::Salt { temp: st }, Dot::Water { temp: _, salinity: sal }) => {
              if *sal <= 100 {
                *sal += 100;
                self.dots[y][x] = Dot::Empty { temp: st }
              }
            },
            (Dot::Water { temp: t, salinity: s1 }, Dot::Water { temp: _, salinity: s2 }) => {
              let s = (s1 + *s2) / 2;
              if s != 0 {
                *s2 = s;
                self.dots[y][x] = Dot::Water { temp: t, salinity: s };
              }
            },
            _ => {}

          }
        }
      }
    }
  }

  pub fn step_fall(&mut self, _dt: &Duration) {
    let mut horiz : Vec<(isize, isize)> = Vec::with_capacity(self.width());

    for y in (0..self.height() as isize).rev() {
      for x in 0..self.width() as isize {
        let dot = self.dots[y as usize][x as usize];
        // if a dot doesnt move, dont bother looking at it
        if !dot.is_fallable() {
          continue;
        }

        let opts = self.fall_neighbors((x, y));

        for (nx, ny) in opts {
          if nx < 0 || ny < 0 {
            continue;
          }

          let mut moved = false;
          if let Some(neighbor) = self.dots.get(ny as usize).and_then(|row| row.get(nx as usize)) {
            if neighbor.is_displaceable_by(&dot) {
              self.dots[y as usize][x as usize] = *neighbor;
              self.dots[ny as usize][nx as usize] = dot;
              moved = true;
              break;
            }
          }

          if !moved && dot.is_flowable() {
            horiz.push((x, y));
          }
        }
      }

      // process horizontal movement
      for &(px, py) in &horiz {
        let opts = self.flow_neighbors((px, py));
        let dot = self.dots[py as usize][px as usize];

        for (nx, ny) in opts {
          if let Some(neighbor) = self.dots.get(ny as usize).and_then(|row| row.get(nx as usize)) {
            if neighbor.is_displaceable_by(&dot) {
              self.dots[py as usize][px as usize] = *neighbor;
              self.dots[ny as usize][nx as usize] = dot;
              break;
            }
          }
        }
      }
      horiz.clear();
    }
  }
}
