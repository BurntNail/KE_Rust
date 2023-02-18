use std::ops::Index;

fn main() {
    //From sample 1 of https://codegolf.stackexchange.com/questions/430/drawing-a-gradient-in-ascii-art
    const ASCII: &[u8] = " .:;+=xX$&".as_bytes();

    const WIDTH: usize = 50;
    const HEIGHT: usize = 35;

    const RE_START: f32 = -2.0;
    const RE_END: f32 = 1.0;
    const IM_START: f32 = -1.0;
    const IM_END: f32 = 1.0;

    let mut v = Vec::with_capacity(WIDTH * HEIGHT);
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let real = (x as f32 / WIDTH as f32).mul_add(RE_END - RE_START, RE_START);
            let imaginary = (y as f32 / HEIGHT as f32).mul_add(IM_END - IM_START, IM_START);

            let mut re: f32 = 0.0;
            let mut im: f32 = 0.0;
            let mut result = 0;
            while re.hypot(im) < 2.0 && result < 500 {
                re = re * re - im * im + real;
                im = re * im + re * im + imaginary;
                result += 1;
            }

            v.push(result);
        }
    }

    let max = *v.iter().max().unwrap();
    let min = *v.iter().min().unwrap();
    let diff = (max - min) as f32;

    let mut s = String::new();
    let mut row_pos = 0;
    for result in v {
        let mapped = result - min;
        let pos = mapped as f32 / diff;
        dbg!(pos);
        let index = (pos * (ASCII.len() - 1) as f32) as usize;
        s += &String::from(ASCII[index] as char);

        row_pos += 1;
        if row_pos == WIDTH {
            row_pos = 0;
            s += "\n"
        }
    }

    println!("{s}");
}
