
pub fn solve_quadratic(a: f32, b: f32, c: f32) -> (Option<f32>, Option<f32>) {
    let x0;
    let x1;
    let discr = b * b - 4.0 * a * c;
    if discr < 0.0 {
        return (None, None);
    } else if discr == 0.0 {
        x0 = Some(-0.5 * b / a);
        x1 = Some(-0.5 * b / a);
    } else {
        let q = match b > 0.0 {
            true    => -0.5 * (b + discr.sqrt()),
            false   => -0.5 * (b - discr.sqrt()),
        };
        x0 = Some(q / a);
        x1 = Some(c / q);
    }
    (x0, x1)
}
