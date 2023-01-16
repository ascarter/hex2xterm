const I2CV: [u8; 6] = [0, 0x5f, 0x87, 0xaf, 0xd7, 0xff];

/*
* This is mostly a rusts translation of
* https://stackoverflow.com/questions/11765623/convert-hex-to-closest-x11-color-number 
* Original gist: https://github.com/Th3Whit3Wolf/space-nvim/issues/1#issue-782680859
*/

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if !args.is_empty() && args[0].len() == 6 {
        hex_to_x256(&args[0]);
    } else if !args.is_empty() && args[0].len() == 7 && &args[0][0..1] == "#" {
        hex_to_x256(&args[0][1..7]);
    } else {
        println!("Please pass in a hexadecimal color")
    }
}

fn hex_val(c: u8, idx: usize) -> Result<u8, ()> {
    match c {
        b'A'..=b'F' => Ok(c - b'A' + 10),
        b'a'..=b'f' => Ok(c - b'a' + 10),
        b'0'..=b'9' => Ok(c - b'0'),
        _ => Err(eprintln!("Invalid character {:?} at position {}", c, idx)),
    }
}

fn hex_to_num(s: &str) -> u8 {
    let mut num = 0u8;
    for (idx, &c) in s.as_bytes().iter().enumerate() {
        if idx == 0 {
            num += hex_val(c, idx).unwrap() * 16;
        } else {
            num += hex_val(c, idx).unwrap();
        }
    }
    num
}

// Calculate the nearest 0-based color index at 16 .. 231
fn v2ci(v: u8) -> u8 {
    if v < 48 {
        0
    } else if (48..115).contains(&v) {
        1
    } else {
        (v - 35) / 40
    }
}

// Return the one which is nearer to the original input rgb value
fn dist_square(a1: u8, b1: u8, c1: u8, a2: u8, b2: u8, c2: u8) -> u8 {
    ((a1 as i32 - a2 as i32) * (a1 as i32 - a2 as i32)
        + (b1 as i32 - b2 as i32) * (b1 as i32 - b2 as i32)
        + (c1 as i32 - c2 as i32) * (c1 as i32 - c2 as i32)) as u8
}

// Convert RGB24 to xterm-256 8-bit value
// For simplicity, assume RGB space is perceptually uniform.
// There are 5 places where one of two outputs needs to be chosen when the
// input is the exact middle:
// - The r/g/b channels and the gray value: the higher value output is chosen.
// - If the gray and color have same distance from the input - color is chosen.
fn hex_to_x256(s: &str) -> u8 {
    println!("Hexadecimal: #{}", &s);
    let r = hex_to_num(&s[0..2]);
    let g = hex_to_num(&s[2..4]);
    let b = hex_to_num(&s[4..6]);
    println!("Red: {}\nGreen: {}\nBlue: {}", r, g, b);

    // 0..5 each
    let ir = v2ci(r);
    let ig = v2ci(g);
    let ib = v2ci(b);

    /* 0..215, lazy evaluation */
    let color_index = 36 * ir + 6 * ig + ib;

    // Calculate the nearest 0-based gray index at 232 .. 255
    let average = ((r as u32 + g as u32 + b as u32) / 3) as u8;
    // 0..23
    let gray_index = if average > 238 {
        23
    } else {
        (average - 3) / 10
    };

    // Calculate the represented colors back from the index
    // r/g/b, 0..255 each
    let cr = I2CV[ir as usize];
    let cg = I2CV[ig as usize];
    let cb = I2CV[ib as usize];

    // same value for r/g/b, 0..255
    let gv = 8 + 10 * gray_index;

    let color_err = dist_square(cr, cg, cb, r, g, b);
    let gray_err = dist_square(gv, gv, gv, r, g, b);

    let x256 = if color_err <= gray_err {
        16 + color_index
    } else {
        232 + gray_index
    };

    println!("xterm256: {}", x256);
    x256
}
