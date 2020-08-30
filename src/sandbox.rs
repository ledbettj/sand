use crate::dot::Dot;
use std::time::Duration;

pub struct Sandbox {
  dots: Vec<Vec<Dot>>
}

impl Sandbox {
  pub fn new(w: usize, h: usize) -> Sandbox {
    let mut dots = Vec::with_capacity(h);

    for _ in 0..h {
      dots.push((0..w).map(|_| Dot::default()).collect());
    }

    Sandbox { dots }
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

  fn fall_neighbors(pos: (isize, isize)) -> Vec<(isize, isize)> {
    let mut result = Vec::with_capacity(5);

    result.push((pos.0, pos.1 + 1));

    if rand::random() {
      result.push((pos.0 - 1, pos.1 + 1));
      result.push((pos.0 + 1, pos.1 + 1));
    } else {
      result.push((pos.0 + 1, pos.1 + 1));
      result.push((pos.0 - 1, pos.1 + 1));
    }
    result
  }

  fn flow_neighbors(pos: (isize, isize)) -> Vec<(isize, isize)> {
    let mut result = Vec::with_capacity(5);

    if rand::random() {
      result.push((pos.0 - 1, pos.1));
      result.push((pos.0 + 1, pos.1));
    } else {
      result.push((pos.0 + 1, pos.1));
      result.push((pos.0 - 1, pos.1));
    }
    result
  }


  pub fn step(&mut self, _dt: &Duration) {
    let mut horiz : Vec<(isize, isize)> = Vec::new();

    for y in (0..self.height() as isize).rev() {
      for x in 0..self.width() as isize {
        let dot = self.dots[y as usize][x as usize];
        // if a dot doesnt move, dont bother looking at it
        if !dot.is_fallable() {
          continue;
        }

        let opts = Sandbox::fall_neighbors((x, y));

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
        let opts = Sandbox::flow_neighbors((px, py));
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
