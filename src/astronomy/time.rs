/// Julian Day Number from a UTC calendar date + fractional hour.
pub fn julian_day(year: i32, month: u32, day: u32, hour_frac: f64) -> f64 {
    let (y, m) = if month <= 2 {
        (year as f64 - 1.0, month as f64 + 12.0)
    } else {
        (year as f64, month as f64)
    };
    let a = (y / 100.0).floor();
    let b = 2.0 - a + (a / 4.0).floor();
    (365.25 * (y + 4716.0)).floor()
        + (30.6001 * (m + 1.0)).floor()
        + day as f64
        + hour_frac / 24.0
        + b
        - 1524.5
}

/// Julian centuries from J2000.0
pub fn j2000_centuries(jd: f64) -> f64 {
    (jd - 2451545.0) / 36525.0
}

/// Greenwich Mean Sidereal Time in degrees [0, 360).
/// IAU 1982 formula.
pub fn gmst_deg(jd: f64) -> f64 {
    let t = j2000_centuries(jd);
    let theta = 280.46061837
        + 360.98564736629 * (jd - 2451545.0)
        + 0.000387933 * t * t
        - t * t * t / 38710000.0;
    reduce_deg(theta)
}

/// Local Sidereal Time in degrees [0, 360).
/// `lon_east` is geographic longitude in degrees, positive east.
pub fn lst_deg(jd: f64, lon_east: f64) -> f64 {
    reduce_deg(gmst_deg(jd) + lon_east)
}

/// Reduce angle to [0, 360).
pub fn reduce_deg(a: f64) -> f64 {
    let r = a % 360.0;
    if r < 0.0 { r + 360.0 } else { r }
}

/// Obliquity of the ecliptic (degrees), IAU formula.
pub fn obliquity(t: f64) -> f64 {
    23.439291111 - 0.013004167 * t - 0.000000164 * t * t + 0.000000504 * t * t * t
}
