use image::{Rgb, ImageBuffer};
use rand::prelude::*;
use rand_distr::{Binomial, Distribution};

const SURROUNDED: f64 = 0.9687f64;
const SUPPORTED: f64  = 0.8145f64;

fn diamond_square(rng: &mut impl Rng, diamond_blur: u8, square_blur: u8, arr: &[u8], width: usize, height: usize) -> Box<[u8]> {
    let mut new_arr = vec![0; width * height * 4].into_boxed_slice();

    let arr_idx     = |x, y| { x + y*width   };
    let new_arr_idx = |x, y| { x + y*width*2 };

    // println!("new-call with {:?}", arr);

    for x in 0..width {
        for y in 0..height {
            new_arr[new_arr_idx(2*x, 2*y)] = arr[arr_idx(x, y)];
        }
    }

    // println!("fresh {:?}", new_arr);

    let surrounded     = Binomial::new(255, SURROUNDED    ).expect("valid distribution");
    let supported      = Binomial::new(255, SUPPORTED     ).expect("valid distribution");
    let balanced       = Binomial::new(255, 0.5           ).expect("valid distribution");
    let alt_supported  = Binomial::new(255, 1.0-SUPPORTED ).expect("valid distribution");
    let alt_surrounded = Binomial::new(255, 1.0-SURROUNDED).expect("valid distribution");

    let mut interpolate = |a: u8, b: u8, c: u8, d: u8, blur: u8| {
        let decided_a = rng.random::<u8>() <= a;
        let decided_b = rng.random::<u8>() <= b;
        let decided_c = rng.random::<u8>() <= c;
        let decided_d = rng.random::<u8>() <= d;
        let interpolated = match (decided_a as u8) + (decided_b as u8) + (decided_c as u8) + (decided_d as u8) {
            0 => alt_surrounded.sample(rng),
            1 => alt_supported.sample(rng),
            2 => balanced.sample(rng),
            3 => supported.sample(rng),
            4 => surrounded.sample(rng),
            _ => unreachable!()
        };
        let average = ((a as u64) + (b as u64) + (c as u64) + (d as u64)) >> 2;
        ((interpolated * (256 - (blur as u64)) + average * (blur as u64)) >> 8) as u8
    };

    // diamond
    
    for x in 0..width {
        for y in 0..width {
            new_arr[new_arr_idx(2*x + 1, 2*y + 1)] = interpolate(
                new_arr[new_arr_idx(2*x, 2*y)],
                new_arr[new_arr_idx((2*x + 2) % width*2, 2*y)],
                new_arr[new_arr_idx(2*x, (2*y + 2) % height*2)],
                new_arr[new_arr_idx((2*x + 2) % width*2, (2*y + 2) % height*2)],
                diamond_blur
            );
        }
    }

    // println!("diamond {:?}", new_arr);

    // square

    for x in 0..width {
        for y in 0..width {
            new_arr[new_arr_idx(2*x + 1, 2*y)] = interpolate(
                new_arr[new_arr_idx(2*x, 2*y)],
                new_arr[new_arr_idx(2*x + 1, 2*y + 1)],
                new_arr[new_arr_idx((2*x + 2) % width*2, 2*y)],
                new_arr[new_arr_idx(2*x + 1, (2*y + height*2 - 1) % height*2)],
                square_blur
            );
            new_arr[new_arr_idx(2*x, 2*y + 1)] = interpolate(
                new_arr[new_arr_idx(2*x, 2*y)],
                new_arr[new_arr_idx(2*x + 1, 2*y + 1)],
                new_arr[new_arr_idx(2*x, (2*y + 1) % height*2)],
                new_arr[new_arr_idx((2*x + width*2 - 1) % width*2, 2*y + 1)],
                square_blur
            );
        }
    }

    // println!("square {:?}", new_arr);

    new_arr
}

fn main() {
    let mut rng = rand::rng();
    let mut arr: Box<[u8]> = Box::new([0, 128, 128, 255]);
    let mut width = 2;
    let mut height = 2;

    for _ in 0..8 {
        arr = diamond_square(&mut rng, 0, 0, &*arr, width, height);
        width *= 2;
        height *= 2;
    }

    let mut img = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(width as u32, height as u32);
    for x in 0..width {
        for y in 0..height {
            img.put_pixel(x as u32, y as u32, Rgb([arr[x + width * y]; 3]));
        }
    }

    img.save("noise.bmp").expect("compatible format");
}
