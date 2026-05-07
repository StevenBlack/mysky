use super::time::{j2000_centuries, obliquity, reduce_deg};

/// Returns (RA degrees, Dec degrees) for the Sun at the given JD (UT).
/// Accuracy: ~0.01° — Meeus Ch 25 low-precision method.
pub fn sun_position(jd: f64) -> (f64, f64) {
    let t = j2000_centuries(jd);

    // Geometric mean longitude (degrees)
    let l0 = reduce_deg(280.46646 + 36000.76983 * t + 0.0003032 * t * t);

    // Mean anomaly (degrees)
    let m = reduce_deg(357.52911 + 35999.05029 * t - 0.0001537 * t * t);
    let m_r = m.to_radians();

    // Equation of center
    let c = (1.914602 - 0.004817 * t - 0.000014 * t * t) * m_r.sin()
        + (0.019993 - 0.000101 * t) * (2.0 * m_r).sin()
        + 0.000289 * (3.0 * m_r).sin();

    // True longitude and apparent longitude (nutation + aberration)
    let sun_true_lon = l0 + c;
    let omega = reduce_deg(125.04 - 1934.136 * t);
    let lambda = sun_true_lon - 0.00569 - 0.00478 * omega.to_radians().sin();

    let lambda_r = lambda.to_radians();
    let eps = obliquity(t) + 0.00256 * omega.to_radians().cos();
    let eps_r = eps.to_radians();

    let ra = f64::atan2(eps_r.cos() * lambda_r.sin(), lambda_r.cos()).to_degrees();
    let ra = reduce_deg(ra);
    let dec = (eps_r.sin() * lambda_r.sin()).clamp(-1.0, 1.0).asin().to_degrees();

    (ra, dec)
}
