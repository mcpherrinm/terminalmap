use io::Reader;
use libc::{c_int};

const chars: &'static [char] = &[
  ' ',
  '\u2597',
  '\u2596',
  '\u2584', // LOWER HALF BLOCK
  '\u259D',
  '\u2590',
  '\u259E',
  '\u259F',
  '\u2598',
  '\u259A',
  '\u258C',
  '\u2599',
  '\u2580',
  '\u259C',
  '\u259B',
  '\u2588'];

fn charmap(pixels: [u8 * 4]) -> char {
  let pixels = vec::map(pixels, |&p| {
    if p > 0 {
      1
    } else {
      0
    }
  });
  let lookup = pixels[0] << 3 | pixels[1] << 2 | pixels[2] << 1 | pixels[3];
  chars[lookup]
}

fn dump(image: &[~[u8]])  {
  io::print("\u001B[H");
  let mut i = 0u;
  let mut buf = ~"\u001B[H\u001B[2J";
  while i < image.len()-1 {
    let mut j = 0u;
    while j < image[i].len()-1 {
      buf.push_char(
                charmap([image[i][j],
                         image[i][j+1],
                         image[i+1][j],
                         image[i+1][j+1]]));
      j+=2u;
    }
    buf.push_char('\n');
    i+=2;
  }
  io::print(buf);
}

fn line(p1: (int, int), p2: (int, int), draw: fn(int, int)) {
  // http://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm#Simplification
  let mut (x1, y1) = p1;
  let (x2, y2) = p2;
  let dx = int::abs(x1-x2);
  let dy = int::abs(y1-y2);
  let sx = if x1<x2 { 1 } else { -1 };
  let sy = if y1<y2 { 1 } else { -1 };

  let mut err = dx-dy;

  loop {
    draw(x1, y1);
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

extern mod unbuffered {
  fn unbuffer();
  fn restore();
  fn getbyte() -> libc::c_int;
}

fn unbuffer() {
  unsafe { unbuffered::unbuffer() }
}

fn restore() {
  unsafe { unbuffered::restore() }
}

fn getbyte() ->i32 {
  unsafe {unbuffered::getbyte()}
}

fn zap(p: (int, int), size: (int, int)) {
  let (cornerx, cornery) = size;
  let mut image: ~[~[u8]] = ~[];
  for int::range(0, cornery) |_| { 
    let mut row: ~[u8] = ~[];
    for int::range(0, cornerx) |_| { row.push(0u8) }
    image.push(row);
  }
  let draw = |x: int, y: int| {
    image[y][x] = 1;
  };

  line(p, (0,0), draw);
  line(p, (cornerx-1,0), draw);
  line(p, (0,cornery-1), draw);
  line(p, (cornerx-1,cornery-1), draw);
  dump(image);
}

fn main() {
  unbuffer();
  let mut i: c_int = 0;
  let mut state = 0;
  let mut x = 15;
  let mut y = 15;
  let mut xb: ~str = ~"";
  let mut yb: ~str = ~"";
  let mut sizex = 120*2;
  let mut sizey = 25*2;
  io::print("\u001B[2J\u001B[?25l\u001B[999;999H\u001B[6n\u001B[H");
  zap((x,y), (sizex, sizey));
  while i >= 0 && i != 4 {
    i = getbyte();
    match (state, i as char) {
      (0, 0x1B as char) => state = 1,
      (1, '[') => state = 2,
      (2, 'A') => {
        state = 0;
        y = int::max(y-1, 0);
        zap((x,y), (sizex, sizey));
      },
      (2, 'B') => {
        state = 0;
        y = int::min(y+1, sizey-1);
        zap((x,y), (sizex, sizey));
      },
      (2, 'C') => {
        state = 0;
        x = int::min(x+2, sizex-1);
        zap((x,y), (sizex, sizey));
      },
      (2, 'D') => {
        state = 0;
        x = int::max(x-2, 0);
        zap((x,y), (sizex, sizey));
      },
      (2 .. 3, '0' .. '9') => {
        state = 3;
        yb.push_char(i as char);
      }
      (3, ';') => state = 4,
      (4, '0' .. '9') => xb.push_char(i as char),
      (4, 'R') => {
        io::println(xb);
        io::println(yb);
        sizex = 2*int::from_str(xb).unwrap()-1;
        sizey = 2*int::from_str(yb).unwrap()-1;
        state = 0; xb = ~""; yb = ~"";
        zap((x,y), (sizex, sizey));
      }
      (_, 4 as char) => break,
      (_, _) => {
        state = 0;
      }
    }
  }
  restore();
}
