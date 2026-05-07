use super::time::{j2000_centuries, reduce_deg};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Planet {
    Mercury,
    Venus,
    Mars,
    Jupiter,
    Saturn,
    Uranus,
    Neptune,
}

impl Planet {
    pub fn name(self) -> &'static str {
        match self {
            Planet::Mercury => "Mercury",
            Planet::Venus => "Venus",
            Planet::Mars => "Mars",
            Planet::Jupiter => "Jupiter",
            Planet::Saturn => "Saturn",
            Planet::Uranus => "Uranus",
            Planet::Neptune => "Neptune",
        }
    }

    pub fn all() -> &'static [Planet] {
        &[
            Planet::Mercury, Planet::Venus, Planet::Mars,
            Planet::Jupiter, Planet::Saturn, Planet::Uranus, Planet::Neptune,
        ]
    }
}

/// Keplerian orbital elements at J2000.0 + linear rates per Julian century.
/// Format: [a, da, e, de, i, di, L, dL, peri, dperi, node, dnode]
/// where a = semi-major axis (AU), e = eccentricity, i = inclination (°),
/// L = mean longitude (°), peri = longitude of perihelion (°),
/// node = longitude of ascending node (°)
fn orbital_elements(planet: Planet) -> [f64; 12] {
    match planet {
        Planet::Mercury => [
            0.38709927, 0.00000037,
            0.20563593, 0.00001906,
            7.00497902, -0.00594749,
            252.25032350, 149472.67411175,
            77.45779628, 0.16047689,
            48.33076593, -0.12534081,
        ],
        Planet::Venus => [
            0.72333566, 0.00000390,
            0.00677672, -0.00004107,
            3.39467605, -0.00078890,
            181.97909950, 58517.81538729,
            131.60246718, 0.00268329,
            76.67984255, -0.27769418,
        ],
        Planet::Mars => [
            1.52371034, 0.00001847,
            0.09339410, 0.00007882,
            1.84969142, -0.00813131,
            -4.55343205, 19140.30268499,
            -23.94362959, 0.44441088,
            49.55953891, -0.29257343,
        ],
        Planet::Jupiter => [
            5.20288700, -0.00011607,
            0.04838624, -0.00013253,
            1.30439695, -0.00183714,
            34.39644051, 3034.74612775,
            14.72847983, 0.21252668,
            100.47390909, 0.20469106,
        ],
        Planet::Saturn => [
            9.53667594, -0.00125060,
            0.05386179, -0.00050991,
            2.48599187, 0.00193609,
            49.95424423, 1222.49514316,
            92.59887831, -0.41897216,
            113.66242448, -0.28867794,
        ],
        Planet::Uranus => [
            19.18916464, -0.00196176,
            0.04725744, -0.00004397,
            0.77263783, -0.00242939,
            313.23810451, 428.48202785,
            170.95427630, 0.40805281,
            74.01692503, 0.04240589,
        ],
        Planet::Neptune => [
            30.06992276, 0.00026291,
            0.00859048, 0.00005105,
            1.77004347, 0.00035372,
            -55.12002969, 218.45945325,
            44.96476227, -0.32241464,
            131.78422574, -0.00508664,
        ],
    }
}

/// Earth's orbital elements (for geocentric conversion)
fn earth_elements() -> [f64; 12] {
    [
        1.00000261, 0.00000562,
        0.01671123, -0.00004392,
        -0.00001531, -0.01294668,
        100.46457166, 35999.37244981,
        102.93768193, 0.32327364,
        0.0, 0.0,
    ]
}

fn solve_kepler(m_deg: f64, e: f64) -> f64 {
    let m_r = m_deg.to_radians();
    let mut ea = m_r;
    for _ in 0..50 {
        let dea = (m_r - ea + e * ea.sin()) / (1.0 - e * ea.cos());
        ea += dea;
        if dea.abs() < 1e-10 {
            break;
        }
    }
    ea
}

/// Heliocentric ecliptic rectangular coordinates (AU) at epoch T (Julian centuries from J2000).
fn heliocentric_xyz(els: &[f64; 12], t: f64) -> (f64, f64, f64) {
    let a     = els[0] + els[1] * t;
    let e     = els[2] + els[3] * t;
    let i_deg = els[4] + els[5] * t;
    let l_deg = els[6] + els[7] * t;
    let w_deg = els[8] + els[9] * t;  // longitude of perihelion
    let n_deg = els[10] + els[11] * t; // longitude of ascending node

    // Mean anomaly
    let m = reduce_deg(l_deg - w_deg);
    // Argument of perihelion
    let omega = reduce_deg(w_deg - n_deg);

    let ea = solve_kepler(m, e);

    // True anomaly
    let nu = 2.0 * f64::atan2(
        ((1.0 + e) / (1.0 - e)).sqrt() * (ea / 2.0).tan(),
        1.0,
    ).to_degrees();

    let r = a * (1.0 - e * ea.cos());

    // Heliocentric ecliptic coords
    let i_r = i_deg.to_radians();
    let n_r = n_deg.to_radians();
    let u = (nu + omega).to_radians();

    let x = r * (n_r.cos() * u.cos() - n_r.sin() * u.sin() * i_r.cos());
    let y = r * (n_r.sin() * u.cos() + n_r.cos() * u.sin() * i_r.cos());
    let z = r * (u.sin() * i_r.sin());

    (x, y, z)
}

/// Returns (RA degrees, Dec degrees) for a planet at the given JD (UT).
pub fn planet_position(jd: f64, planet: Planet) -> (f64, f64) {
    let t = j2000_centuries(jd);

    let (px, py, pz) = heliocentric_xyz(&orbital_elements(planet), t);
    let (ex, ey, ez) = heliocentric_xyz(&earth_elements(), t);

    // Geocentric ecliptic
    let gx = px - ex;
    let gy = py - ey;
    let gz = pz - ez;

    // Ecliptic to equatorial (J2000 obliquity ≈ 23.43929°)
    let eps = (23.439291111 - 0.013004167 * t).to_radians();
    let qx = gx;
    let qy = gy * eps.cos() - gz * eps.sin();
    let qz = gy * eps.sin() + gz * eps.cos();

    let ra = reduce_deg(f64::atan2(qy, qx).to_degrees());
    let dec = f64::atan2(qz, (qx * qx + qy * qy).sqrt()).to_degrees();

    (ra, dec)
}
