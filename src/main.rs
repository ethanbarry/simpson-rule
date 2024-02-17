fn main() {
    // Define the integrand as a closure accepting one `f64` value.
    let integrand = |x: f64| x.exp();

    let integral = int_simpson(0_f64, 1_f64, 10_000_000, integrand);

    println!("Result: {}", integral);
}

/// Integrates using Simpson's rule.
///
/// ## Usage
///
/// Takes the bounds of integration `a` and `b`, as well as the
/// precision `n` (i.e. number of subdivisions of the integral to perform)
/// and the integrand, which is a closure accepting one f64 value as
/// the input, and returning ditto.
///
/// ## Example
///
/// ```rust
/// let f = |x: f64| x.exp();
/// let area = int_simpson(0f64, 1f64, 100, f);
/// let abs_diff = (area - 2.718281828459_f64).abs();
/// assert!(abs_diff < 1e-10);
/// ```
fn int_simpson<F>(a: f64, b: f64, n: i32, integrand: F) -> f64
where
    F: Fn(f64) -> f64,
{
    // We want to sum from a to b, in n subintervals, where the width delta_x is (a - b) / n.
    // Each subinterval is [a, a+delta_x] up to b.
    let delta_x = (b - a) / n as f64;
    let mut result = 0.0_f64;
    for i in 1..=n {
        result += delta_x
            * (integrand(a + (i as f64 - 1_f64) * delta_x)
                + 4_f64 * integrand(a - delta_x / 2_f64 + i as f64 * delta_x)
                + integrand(a + i as f64 * delta_x))
            / 6_f64;
    }
    result
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn check_difference() {
        let f = |x: f64| x.exp();
        let area = int_simpson(0f64, 1f64, 10_000_000, f);
        let abs_diff = (area - 1.718281828_f64).abs();
        assert!(abs_diff < 1e-8);
    }
}
