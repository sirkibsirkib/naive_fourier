extern crate simple_vector2d;
extern crate textplots;

use simple_vector2d::consts::ZERO_F32 as ZERO;
use std::ops::Add;

type Pt = simple_vector2d::Vector2<f32>;

pub fn fourier(samples: &[f32], sample_period: f32, query_frequency: f32) -> f32 {
    let in_step = sample_period / samples.len() as f32;
    samples
        .iter()
        .enumerate()
        .map(|(i, &sample)| {
            let rotations: f32 = query_frequency * i as f32 * in_step;
            let p: Pt = Pt::unit_vector(rotations) * sample;
            p
        })
        .fold(ZERO, Pt::add)
        .length()
        / samples.len() as f32
}

#[cfg(test)]
mod tests {
    use super::*;
    use textplots::{Chart, Plot, Shape};

    fn flt_range(start: f32, step: f32, end: f32) -> impl Iterator<Item = f32> {
        let steps = ((end - start) / step) as usize;
        (0..steps).map(move |x| x as f32 * step + start)
    }

    fn index_of_max(arr: &[f32]) -> usize {
        let mut max_at = 0;
        let mut max = arr[max_at];
        for (i, x) in arr.iter().cloned().enumerate() {
            if x > max {
                max = x;
                max_at = i;
            }
        }
        max_at
    }

    #[test]
    fn it_works() {
        let function = |x: f32| (x * 2.0).sin() + ((x + 1.0) * 5.0).sin();

        let len = 20.0;
        let num_samples = 1200;
        let sx: Vec<f32> = flt_range(0.0, len / num_samples as f32, len).collect();
        let sy: Vec<f32> = sx.iter().cloned().map(function).collect();
        // println!("samples {:?}", &sy);

        println!("SAMPLES");

        {
            let s: Vec<(f32, f32)> = sx.iter().cloned().zip(sy.iter().cloned()).collect();
            Chart::new(300, 100, 0.0, len)
                .lineplot(Shape::Lines(&s[..]))
                .display();
        }
        // return;

        let fx: Vec<f32> = flt_range(0.0, 0.03, 10.0).collect();
        let fy: Vec<f32> = fx.iter().map(|&x| fourier(&sy, len, x)).collect();

        let v: Vec<(f32, f32)> = fx.iter().cloned().zip(fy.iter().cloned()).collect();
        // println!("v {:?}", &v);

        println!("FFT");
        Chart::new(300, 100, 0.0, *fx.iter().last().unwrap())
            .lineplot(Shape::Lines(&v[..]))
            .display();
        println!("MAX AT {}", fx[index_of_max(&fy[..])]);
    }
}
