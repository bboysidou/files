use super::bezier_axis::bezier_axis;

const SOLVER_ITERATIONS: usize = 24;

pub fn cubic_bezier(control: [f32; 4]) -> impl Fn(f32) -> f32 {
    let [x1, y1, x2, y2] = control;
    move |progress| {
        let target = progress.clamp(0.0, 1.0);
        let (mut low, mut high) = (0.0_f32, 1.0_f32);
        for _ in 0..SOLVER_ITERATIONS {
            let middle = (low + high) / 2.0;
            if bezier_axis(middle, x1, x2) < target {
                low = middle;
            } else {
                high = middle;
            }
        }
        bezier_axis((low + high) / 2.0, y1, y2)
    }
}
