use super::*;
use textplots::{Chart, Plot, Shape};

fn f32_range(start: f32, end: f32, samples: usize) -> impl Iterator<Item = f32> + 'static {
    let len = end - start;
    (0..samples).map(move |i| (i as f32 / samples as f32) * len + start)
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
const NUM_SAMPLES: usize = 1024;

#[test]
fn it_works() {
    let function = |x: f32| (x * 2.).sin() + ((x + 1.) * 5.).sin();
    let end = 20.;
    let sx: Vec<f32> = f32_range(0., end, NUM_SAMPLES).collect();
    let sy: Vec<f32> = sx.iter().cloned().map(function).collect();
    println!("Plotting w/ time");
    let points: Vec<(f32, f32)> = sx.iter().copied().zip(sy.iter().copied()).collect();
    Chart::new(300, 100, 0., end)
        .lineplot(Shape::Lines(&points[..]))
        .display();

    let fx: Vec<f32> = f32_range(0., end, 1000).collect();
    let fy: Vec<f32> = fx.iter().map(|&x| fourier(&sy, 25., x)).collect();

    let points: Vec<(f32, f32)> = fx.iter().copied().zip(fy.iter().copied()).collect();

    println!("Plotting w/ frequency");
    Chart::new(300, 100, 0.0, end)
        .lineplot(Shape::Lines(&points[..]))
        .display();
    println!("MAX AT {}", fx[index_of_max(&fy[..])]);
}
