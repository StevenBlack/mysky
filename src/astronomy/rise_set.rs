use super::time::{gmst_deg, julian_day, reduce_deg};

/// Standard altitude (degrees) for rise/set computation.
pub const H0_STAR: f64 = -0.5667;
pub const H0_SUN: f64 = -0.8333;
pub const H0_MOON: f64 = 0.1253; // approx: 0.7275 * 0.9507 - 0.5667

pub struct RiseTransitSet {
    /// Minutes from midnight UTC
    pub rise: Option<f64>,
    pub transit: Option<f64>,
    pub set: Option<f64>,
    pub circumpolar: bool,
    pub never_rises: bool,
}

/// Compute rise, transit, and set times for a body with given RA/Dec.
/// `jd0` is the Julian Day for 0h UT of the date of interest.
/// `lat`, `lon_east` are observer coords (degrees).
/// `h0` is the standard altitude for this body type.
///
/// Returns times as minutes from 0h UT (local conversion done in app layer).
/// Single-iteration algorithm (accurate to ~1 min for stars/planets, ~2 min for Moon).
pub fn rise_transit_set(
    ra: f64,
    dec: f64,
    lat: f64,
    lon_east: f64,
    jd0: f64,
    h0: f64,
) -> RiseTransitSet {
    let lat_r = lat.to_radians();
    let dec_r = dec.to_radians();

    // Hour angle at rise/set
    let cos_h0 = (h0.to_radians().sin() - lat_r.sin() * dec_r.sin())
        / (lat_r.cos() * dec_r.cos());

    if cos_h0 < -1.0 {
        return RiseTransitSet {
            rise: None, transit: None, set: None,
            circumpolar: true, never_rises: false,
        };
    }
    if cos_h0 > 1.0 {
        return RiseTransitSet {
            rise: None, transit: None, set: None,
            circumpolar: false, never_rises: true,
        };
    }

    let h0_deg = cos_h0.acos().to_degrees();

    // GMST at 0h UT of the date
    let theta0 = gmst_deg(jd0);

    // Approximate transit time (fraction of day, 0h UT = 0, 24h UT = 1)
    let m_transit = reduce_deg(ra - lon_east - theta0) / 360.0;
    let m_rise    = m_transit - h0_deg / 360.0;
    let m_set     = m_transit + h0_deg / 360.0;

    let to_min = |m: f64| -> f64 {
        let norm = m - m.floor(); // [0,1)
        norm * 1440.0             // minutes in a day
    };

    RiseTransitSet {
        rise: Some(to_min(m_rise)),
        transit: Some(to_min(m_transit)),
        set: Some(to_min(m_set)),
        circumpolar: false,
        never_rises: false,
    }
}

/// JD for 0h UTC on the given calendar date.
pub fn jd_midnight(year: i32, month: u32, day: u32) -> f64 {
    julian_day(year, month, day, 0.0)
}
