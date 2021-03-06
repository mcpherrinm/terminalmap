extern crate libc;
use std::cmp::{min, max};
use std::num::SignedInt;


fn charmap(mut pixels: [u8, ..4]) -> char {
  static CHARS: &'static [char] = &[
    ' ',
    '\u{2597}',
    '\u{2596}',
    '\u{2584}', // LOWER HALF BLOCK
    '\u{259D}',
    '\u{2590}',
    '\u{259E}',
    '\u{259F}',
    '\u{2598}',
    '\u{259A}',
    '\u{258C}',
    '\u{2599}',
    '\u{2580}',
    '\u{259C}',
    '\u{259B}',
    '\u{2588}'];
  for i in range(0, 4u) {
    pixels[i] = if pixels[i] > 0 { 1 } else { 0 };
  }
  let lookup = pixels[0] << 3 | pixels[1] << 2 | pixels[2] << 1 | pixels[3];
  CHARS[lookup as uint]
}

fn dump(image: &[Vec<u8>])  {
  println!("\u{001B}[H");
  let mut i = 0u;
  let mut buf = String::from_str("\u{001B}[H\u{001B}[2J");
  while i < image.len()-1 {
    let mut j = 0u;
    while j < image[i].len()-1 {
      buf.push(
                charmap([image[i][j],
                         image[i][j+1],
                         image[i+1][j],
                         image[i+1][j+1]]));
      j+=2u;
    }
    buf.push('\n');
    i+=2;
  }
  println!("{}", buf);
}

fn line(p1: (int, int), p2: (int, int), image: &mut [Vec<u8>]) {
  // http://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm#Simplification
  let (mut x1, mut y1) = p1;
  let (x2, y2) = p2;
  let dx = (x1-x2).abs();
  let dy = (y1-y2).abs();
  let sx = if x1<x2 { 1 } else { -1 };
  let sy = if y1<y2 { 1 } else { -1 };

  let mut err = dx-dy;

  loop {
    image[y1 as uint][x1 as uint] = 1;
    if x1 == x2 && y1 == y2 { break; }
    let e2 = err * 2;
    if e2 > -dy {
      err = err - dy;
      x1 += sx;
    }
    if e2 < dx {
      err = err + dx;
      y1 += sy;
    }
  }
}

mod unbuffered {
	#[link(name="unbuffered")]
	extern  {
	  pub fn unbuffer();
	  pub fn restore();
	  pub fn getbyte() -> ::libc::c_int;
	}
}

fn unbuffer() {
  unsafe { unbuffered::unbuffer() }
}

fn restore() {
  unsafe { unbuffered::restore() }
}

fn getbyte() -> u8 {
  unsafe {unbuffered::getbyte() as u8}
}

fn zap(p: (int, int), size: (int, int)) {
  let (cornerx, cornery) = size;
  let mut image: Vec<Vec<u8>> = Vec::new();
  for _ in range(0, cornery) { 
    let mut row: Vec<u8> = Vec::new();
    for _ in range(0, cornerx) { row.push(0u8) }
    image.push(row);
  }

  line(p, (0,0), image.as_mut_slice());
  line(p, (cornerx-1,0), image.as_mut_slice());
  line(p, (0,cornery-1), image.as_mut_slice());
  line(p, (cornerx-1,cornery-1), image.as_mut_slice());
  dump(image.as_slice());
}

fn main() {
  unbuffer();
  let mut i = 0;
  let mut state = 0i;
  let mut x = 15;
  let mut y = 15;
  let mut xb = String::new();
  let mut yb = String::new();
  let mut sizex = 120*2;
  let mut sizey = 25*2;
  std::io::stdio::print("\u{001B}[2J\u{001B}[?25l\u{001B}[999;999H\u{001B}[6n\u{001B}[H");
  zap((x,y), (sizex, sizey));
  while i != 4 {
    i = getbyte();
    match (state, i) {
      (0, 0x1B) => state = 1,
      (1, 0x5B) => state = 2,
      (2, 0x41) => {
        state = 0;
        y = max(y-1, 0);
        zap((x,y), (sizex, sizey));
      },
      (2, 0x42) => {
        state = 0;
        y = min(y+1, sizey-1);
        zap((x,y), (sizex, sizey));
      },
      (2, 0x43) => {
        state = 0;
        x = min(x+2, sizex-1);
        zap((x,y), (sizex, sizey));
      },
      (2, 0x44) => {
        state = 0;
        x = max(x-2, 0);
        zap((x,y), (sizex, sizey));
      },
      (2 ... 3, 0x30 ... 0x39) => {
        state = 3;
        yb.push(i as char);
      }
      (3, 0x3B) => state = 4,
      (4, 0x30 ... 0x39) => xb.push(i as char),
      (4, 0x52) => {
        println!("{}", xb);
        println!("{}", yb);
        sizex = 2*from_str(xb.as_slice()).unwrap()-1;
        sizey = 2*from_str(yb.as_slice()).unwrap()-1;
        state = 0; xb = String::new(); yb = String::new();
        zap((x,y), (sizex, sizey));
      }
      (_, 4) => break,
      (_, _) => {
        state = 0;
      }
    }
  }
  restore();
}
