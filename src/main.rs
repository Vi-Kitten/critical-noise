use image::{Rgb, ImageBuffer};
use rand::prelude::*;
use rand_distr::{Beta, Distribution};

const SURROUNDED: f64 = 0.9687f64;
const SUPPORTED: f64  = 0.8145f64;

fn diamond_square(rng: &mut impl Rng, diamond_sharpness: f64, square_sharpness: f64, arr: &[f64], width: usize, height: usize) -> Box<[f64]> {
    let mut new_arr = vec![0.0; width * height * 4].into_boxed_slice();

    let arr_idx     = |x, y| { (x % width) + (y % height)*width   };
    let new_arr_idx = |x, y| { (x % (width*2)) + (y % (height*2))*width*2 };

    // println!("new-call with {:?}", arr);

    for x in 0..width {
        for y in 0..height {
            new_arr[new_arr_idx(2*x, 2*y)] = arr[arr_idx(x, y)];
        }
    }

    // println!("fresh {:?}", new_arr);

    let surrounded     = Beta::new(SURROUNDED, 1.0-SURROUNDED).expect("valid distribution");
    let supported      = Beta::new(SUPPORTED,   1.0-SUPPORTED).expect("valid distribution");
    let balanced       = Beta::new(0.5,           0.5        ).expect("valid distribution");
    let alt_supported  = Beta::new(1.0-SUPPORTED,   SUPPORTED).expect("valid distribution");
    let alt_surrounded = Beta::new(1.0-SURROUNDED, SURROUNDED).expect("valid distribution");

    let mut interpolate = |center: f64, ring: f64, corners: f64, sharpness: f64| {
        let interpolated = match center.round() as u8 {
            0 => alt_surrounded.sample(rng),
            1 => alt_supported.sample(rng),
            2 => balanced.sample(rng),
            3 => supported.sample(rng),
            4 => surrounded.sample(rng),
            _ => unreachable!()
        };
        let average
            = center * (81.0 / 256.0)
            - ring * (9.0 / 256.0)
            + corners * (1.0 / 256.0);
        interpolated * sharpness + average * (1.0 - sharpness)
    };

    // diamond
    
    for x in 0..width {
        for y in 0..width {
            new_arr[new_arr_idx(2*x + 1, 2*y + 1)] = interpolate(
                new_arr[new_arr_idx(2*x, 2*y)] +
                new_arr[new_arr_idx(2*x + 2, 2*y)] +
                new_arr[new_arr_idx(2*x, 2*y + 2)] +
                new_arr[new_arr_idx(2*x + 2, 2*y + 2)],
            
                new_arr[new_arr_idx(2*x + width*4 - 2, 2*y)] +
                new_arr[new_arr_idx(2*x + width*4 - 2, 2*y + 2)] +
                new_arr[new_arr_idx(2*x + 4, 2*y)] +
                new_arr[new_arr_idx(2*x + 4, 2*y + 2)] +
                new_arr[new_arr_idx(2*x, 2*y + height*4 - 2)] +
                new_arr[new_arr_idx(2*x + 2, 2*y + height*4 - 2)] +
                new_arr[new_arr_idx(2*x, 2*y + height*4 - 2)] +
                new_arr[new_arr_idx(2*x + 2, 2*y + height*4 - 2)],

                new_arr[new_arr_idx(2*x + width*4 - 2, 2*y + width*4 - 2)] +
                new_arr[new_arr_idx(2*x + 4, 2*y + width*4 - 2)] +
                new_arr[new_arr_idx(2*x + width*4 - 2, 2*y + 4)] +
                new_arr[new_arr_idx(2*x + 4, 2*y + 4)],

                diamond_sharpness
            );
        }
    }

    // println!("diamond {:?}", new_arr);

    // square

    for x in 0..width {
        for y in 0..width {
            new_arr[new_arr_idx(2*x + 1, 2*y)] = interpolate(
                new_arr[new_arr_idx(2*x, 2*y)] +
                new_arr[new_arr_idx(2*x + 1, 2*y + 1)] +
                new_arr[new_arr_idx(2*x + 2, 2*y)] +
                new_arr[new_arr_idx(2*x + 1, 2*y + height*4 - 1)],

                new_arr[new_arr_idx(2*x + width*4 - 1, 2*y + height*4 - 1)] +
                new_arr[new_arr_idx(2*x + width*4 - 1, 2*y + 1)] +            
                new_arr[new_arr_idx(2*x + 3, 2*y + height*4 - 1)] +
                new_arr[new_arr_idx(2*x + 3, 2*y + 1)] +
                new_arr[new_arr_idx(2*x, 2*y + height*4 - 2)] +
                new_arr[new_arr_idx(2*x + 2, 2*y + height*4 - 2)] +
                new_arr[new_arr_idx(2*x, 2*y + 2)] +
                new_arr[new_arr_idx(2*x + 2, 2*y + 2)],

                new_arr[new_arr_idx(2*x + width*4 - 2, 2*y)] +
                new_arr[new_arr_idx(2*x + 1, 2*y + 3)] +
                new_arr[new_arr_idx(2*x + 4, 2*y)] +
                new_arr[new_arr_idx(2*x + 1, 2*y + height*4 - 3)],
            
                square_sharpness
            );
            new_arr[new_arr_idx(2*x, 2*y + 1)] = interpolate(
                new_arr[new_arr_idx(2*x, 2*y)] +
                new_arr[new_arr_idx(2*x + 1, 2*y + 1)] +
                new_arr[new_arr_idx(2*x, 2*y + 2)] +
                new_arr[new_arr_idx(2*x + width*4 - 1, 2*y + 1)],

                new_arr[new_arr_idx(2*x + width*4 - 1, 2*y + height*4 - 1)] +
                new_arr[new_arr_idx(2*x + 1, 2*y + height*4 - 1)] +
                new_arr[new_arr_idx(2*x + width*4 - 1, 2*y + 3)] +
                new_arr[new_arr_idx(2*x + 1, 2*y + 3)] +
                new_arr[new_arr_idx(2*x + width*4 - 2, 2*y)] +
                new_arr[new_arr_idx(2*x + width*4 - 2, 2*y + 2)] +
                new_arr[new_arr_idx(2*x + 2, 2*y)] +
                new_arr[new_arr_idx(2*x + 2, 2*y + 2)],

                new_arr[new_arr_idx(2*x, 2*y + height*4 - 2)] +
                new_arr[new_arr_idx(2*x + 3, 2*y + 1)] +
                new_arr[new_arr_idx(2*x, 2*y + 4)] +
                new_arr[new_arr_idx(2*x + width*4 - 3, 2*y + 1)],

                square_sharpness
            );
        }
    }

    // println!("square {:?}", new_arr);

    new_arr
}

fn main() {
    let mut rng = rand::rng();
    let root_dist = Beta::new(0.5, 0.5).expect("valid beta distribution");
    let mut width = 16;
    let mut height = 9;
    let mut arr: Box<[f64]> = root_dist.sample_iter(&mut rng).take(width * height).collect();

    let sharpness = |t: f64| { f64::exp(-t*0.25) };
    
    for _ in 0..4 {
        arr = diamond_square(&mut rng, 1.0, 1.0, &*arr, width, height);
        width *= 2;
        height *= 2;
    }
    
    for t in 0..4 {
        arr = diamond_square(&mut rng, sharpness((t*2) as f64), sharpness((t*2 + 1) as f64), &*arr, width, height);
        width *= 2;
        height *= 2;
    }

    let mut img = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(width as u32, height as u32);
    for x in 0..width {
        for y in 0..height {
            img.put_pixel(x as u32, y as u32, Rgb([(arr[x + width * y] * 256.0).floor() as u8; 3]));
        }
    }

    img.save("noise.bmp").expect("compatible format");
}
