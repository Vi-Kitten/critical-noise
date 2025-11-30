use image::{Rgb, ImageBuffer};
use rand::prelude::*;

fn black() -> Rgb<f32> {
    Rgb([0.0, 0.0, 0.0])
}

fn white() -> Rgb<f32> {
    Rgb([1.0, 1.0, 1.0])
}

fn critical(width: usize, height: usize) -> ImageBuffer<Rgb<f32>, Vec<f32>> {
    let mut rng = rand::rng();

    #[derive(Debug, Clone, Copy)]
    struct FillNode {
        filled: bool,
        next_x: usize,
        next_y: usize,
    }

    let mut fill_loops = Vec::new();
    for x in 0..width {
        let mut column = Vec::new();
        for y in 0..height {
            column.push(FillNode{
                filled: false,
                next_x: x,
                next_y: y
            })
        }
        fill_loops.push(column.into_boxed_slice());
    }
    let mut fill_loops = fill_loops.into_boxed_slice();

    fn iter_mut_loop(
        candidate_x: usize,
        candidate_y: usize,
        loops: &mut Box<[Box<[FillNode]>]>,
        mut f: impl FnMut(&mut FillNode) -> bool
    ) {
        let mut next_x = candidate_x;
        let mut next_y = candidate_y;
        loop {
            let node = &mut loops[next_x][next_y];
            if !f(node) {
                break;
            };
            next_x = node.next_x;
            next_y = node.next_y;
            if (next_x == candidate_x) && (next_y == candidate_y) {
                break;
            }
        }
    }

    fn combine_regions_with(
        candidate_x_a: usize,
        candidate_y_a: usize,
        candidate_x_b: usize,
        candidate_y_b: usize,
        loops: &mut Box<[Box<[FillNode]>]>
    ) {
        let mut is_same = false;
        iter_mut_loop(candidate_x_a, candidate_y_a, loops, |node| {
            is_same |= (node.next_x == candidate_x_b) && (node.next_y == candidate_y_b);
            !is_same
        });
        if is_same {
            return;
        };
        let temp_x: usize = loops[candidate_x_a][candidate_y_a].next_x;
        let temp_y: usize = loops[candidate_x_a][candidate_y_a].next_y;
        let temp_x: usize = std::mem::replace(&mut loops[candidate_x_b][candidate_y_b].next_x, temp_x);
        let temp_y: usize = std::mem::replace(&mut loops[candidate_x_b][candidate_y_b].next_y, temp_y);
        loops[candidate_x_a][candidate_y_a].next_x = temp_x;
        loops[candidate_x_a][candidate_y_a].next_y = temp_y;
    }

    let mut connections: Vec<bool> = vec![false; width as usize];

    for y in 0..height {
        let prev_connections = std::mem::replace(&mut connections, vec![]);

        let mut start_x = 0;
        while start_x < width {
            let mut end_x = start_x + 1;
            'grow: while end_x < width {
                if rng.random_bool(0.5) {
                    combine_regions_with(end_x, y, end_x - 1, y, &mut fill_loops);
                    end_x += 1
                } else {
                    break 'grow;
                }
            }

            for x in start_x..end_x {
                if prev_connections[x] {
                    combine_regions_with(x, y, x, y - 1, &mut fill_loops);
                }
                connections.push(rng.random_bool(0.5))
            }

            start_x = end_x
        }
    }

    let mut image = ImageBuffer::new(width as u32, height as u32);

    for x in 0..height {
        for y in 0..width {
            let colour = if rng.random_bool(0.5) {
                black()
            } else {
                white()
            };
            iter_mut_loop(x, y, &mut fill_loops, |node| {
                if node.filled {
                    return false
                }
                image.put_pixel(node.next_x as u32, node.next_y as u32, colour);
                true
            });
        }
    }

    image
}

fn noise_colour(x: f32) -> Rgb<u8> {
    Rgb([(x * 256.0) as u8; 3])

}

fn main() {
    let width: usize = 100;
    let height: usize = 100;
    let freq: f32 = 20.0;
    let alpha: f32 = 0.5;
    let rounds = 5;
    let octaves = (0..rounds).map(|octave| {
        image::imageops::blur(&critical(width, height), freq * alpha.powi(octave))
    }).collect::<Box<[_]>>();
    let mut noise = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(width as u32, height as u32);
    for x in 0..width {
        for y in 0..height {
            let mut octave_sum = 0.0;
            for octave in 0..rounds {
                octave_sum += octaves[octave as usize].get_pixel(x as u32, y as u32).0[0] * alpha.powi(octave)
            }
            noise.put_pixel(x as u32, y as  u32, noise_colour(octave_sum * (1.0 - alpha)));
        }
    }
    noise.save("noise.bmp").unwrap();
}
