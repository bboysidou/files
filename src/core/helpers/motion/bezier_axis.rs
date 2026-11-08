pub fn bezier_axis(time: f32, first: f32, second: f32) -> f32 {
    let inverse = 1.0 - time;
    3.0 * inverse * inverse * time * first
        + 3.0 * inverse * time * time * second
        + time * time * time
}
