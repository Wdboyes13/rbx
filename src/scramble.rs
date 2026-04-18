use std::fmt::{Display, Write};
use rand::{random_range, random_bool};
use serde::{Serialize, Deserialize};

#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum Face {
    R, L, U, D, B, F
}

fn div_ceil(a: i32, b: i32) -> i32 {
    (a + b - 1) / b
}

impl Face {
    pub fn to_char(self) -> char {
        const FACES: [char; 6] = ['R', 'L', 'U', 'D', 'B', 'F'];
        FACES[self as usize]
    }
    pub fn from_i32(x: i32) -> Self {
        const FACES: [Face; 6] = [Face::R, Face::L, Face::U, Face::D, Face::B, Face::F];
        FACES[x as usize]
    }
}

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct Mods {
    pub x2: bool,
    pub prime: bool,
    pub wide: bool,
    pub slice: Option<u32>
}

impl Mods {
    pub fn rand(x: i32) -> Self {
        let mut ret = Self {
            x2: random_bool(0.5),
            prime: random_bool(0.5),
            wide: false,
            slice: None
        };

        if x >= 4 {
            ret.wide = random_bool(0.5);
        }
        
        if x >= 5 && random_bool(0.5) {
            ret.slice = Some(random_range(3..=div_ceil(x, 2)) as u32);
        }

        // we dont want both lol
        if ret.wide && ret.slice.is_some() {
            let wide_or_slice = random_bool(0.5);
            if wide_or_slice {
                ret.slice = None;
            } else {
                ret.wide = false;
            }
        }

        ret
    }
}

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct Move {
    pub face: Face,
    pub mods: Mods
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Seq {
    pub val: Vec<Move>,
    pub cubex: i32,
    pub len: usize,
}

impl Seq {
    pub fn generate(len: usize, cubex: i32) -> Self {
        let mut sq = Vec::<Move>::with_capacity(len);

        let mut pface: i32 = -1;
        for _ in 0..len {
            let mut face: i32 = random_range(0..6);
            while face == pface {
                face = random_range(0..6);
            }

            sq.push(Move{
                face: Face::from_i32(face),
                mods: Mods::rand(cubex)
            });

            pface = face;
        }

        Self{val: sq, cubex, len}
    }
}

impl Display for Seq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        

        for mv in &self.val {
            if let Some(slice) = mv.mods.slice {
                f.write_str(format!("{}", slice).as_str())?;
            }

            f.write_char(mv.face.to_char())?;

            if mv.mods.wide || mv.mods.slice.is_some() {
                f.write_char('w')?;
            }

            if mv.mods.x2 {
                f.write_char('2')?;
            }

            if mv.mods.prime {
                f.write_char('\'')?;
            }

            f.write_char(' ')?;
        }

        Ok(())
    }
}