use std::default::Default;
use rand::{self, prelude::*};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Dot {
  Empty { temp: i16 },
  Sand { temp: i16 },
  Dirt { temp: i16 },
  Water { temp: i16, salinity: u16 },
  Salt { temp: i16 },
  Iron { temp: i16 }
}

impl Default for Dot {
  fn default() -> Dot {
    Dot::Empty { temp: 0 }
  }
}

impl Dot {

  pub fn next(&self) -> Dot {
    match self {
      Dot::Empty { temp: t } => Dot::Sand { temp: *t },
      Dot::Sand { temp: t }  => Dot::Dirt { temp: *t },
      Dot::Dirt { temp: t }  => Dot::Water { temp: *t, salinity: 0 },
      Dot::Water { temp: t, salinity: _ } => Dot::Salt { temp: *t },
      Dot::Salt { temp: t }  => Dot::Iron { temp: *t },
      Dot::Iron { temp: t }  => Dot::Empty { temp: *t }
    }
  }

  pub fn is_fallable(&self) -> bool {
    match self {
      Dot::Empty { temp: _ } | Dot::Iron { temp: _ } => false,
      _ => true
    }
  }

  pub fn is_flowable(&self) -> bool {
    match self {
      Dot::Water { temp: _, salinity: _ } => true,
      _ => false
    }
  }

  pub fn is_displaceable(&self) -> bool {
    match self {
      Dot::Empty { temp: _ } | Dot::Water { temp: _, salinity: _ } => true,
      _ => false
    }
  }

  pub fn is_displaceable_by(&self, other: &Dot) -> bool {
    if !self.is_displaceable() {
      return false;
    }

    match (self, other) {
      (Dot::Empty { temp: _ }, Dot::Empty { temp: _, }) => false,
      (Dot::Empty { temp: _ }, _) => true,
      (Dot::Water { temp: _, salinity: _ }, Dot::Water { temp: _, salinity: _ }) => false,
      (Dot::Water { temp: _, salinity: _ }, Dot::Empty { temp: _ }) => false,
      (Dot::Water { temp: _, salinity: _ }, _) => true,
      _ => false
    }
  }


  pub fn is_empty(&self) -> bool {
    match self {
      Dot::Empty { temp: _ } => true,
      _ => false
    }
  }

  pub fn draw(&self, frame: &mut [u8]) {
    let c = match self {
      Dot::Empty { temp: _ } => [0x00, 0x00, 0x00, 0xFF],
      Dot::Sand { temp: _ }  => [0xED, 0xC9, 0xAF, 0xFF],
      Dot::Dirt { temp: _ }  => [0xBD, 0x99, 0xCF, 0xFF],
      Dot::Water { temp: _, salinity: s } => [0x00, 0xFF - (*s as u8), 0xFF, 0xFF],
      Dot::Salt { temp: _ }  => [0xFF, 0xFF, 0xFF, 0xFF],
      Dot::Iron { temp: _ }  => [0xCC, 0xCC, 0xCC, 0xFF]
    };

    frame.copy_from_slice(&c);
  }
}
