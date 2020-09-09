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
mod tests;
