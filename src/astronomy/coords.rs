use super::time::reduce_deg;

/// Convert equatorial (RA, Dec) to horizontal (Alt, Az) coordinates.
/// All angles in degrees. `lst` is Local Sidereal Time in degrees.
/// Returns (altitude, azimuth) where Az is measured N→E (compass bearing).
pub fn equatorial_to_horizontal(ra: f64, dec: f64, lat: f64, lst: f64) -> (f64, f64) {
    let ha = (lst - ra).to_radians(); // hour angle
    let dec_r = dec.to_radians();
    let lat_r = lat.to_radians();

    let sin_alt = lat_r.sin() * dec_r.sin() + lat_r.cos() * dec_r.cos() * ha.cos();
    let alt = sin_alt.clamp(-1.0, 1.0).asin().to_degrees();

    // Azimuth from South, through West (astronomical convention)
    let az_s = f64::atan2(ha.sin(), ha.cos() * lat_r.sin() - dec_r.tan() * lat_r.cos())
        .to_degrees();

    // Convert to N→E (compass): add 180°
    let az = reduce_deg(az_s + 180.0);

    (alt, az)
}

/// Convert ecliptic longitude/latitude to RA/Dec. All angles degrees.
#[allow(dead_code)]
pub fn ecliptic_to_equatorial(lon: f64, lat: f64, eps: f64) -> (f64, f64) {
    let lon_r = lon.to_radians();
    let lat_r = lat.to_radians();
    let eps_r = eps.to_radians();

    let ra = f64::atan2(
        eps_r.cos() * lon_r.sin() * lat_r.cos() - eps_r.sin() * lat_r.sin(),
        lon_r.cos() * lat_r.cos(),
    )
    .to_degrees();
    let ra = reduce_deg(ra);

    let dec = (eps_r.sin() * lon_r.sin() * lat_r.cos() + eps_r.cos() * lat_r.sin())
        .clamp(-1.0, 1.0)
        .asin()
        .to_degrees();

    (ra, dec)
}
