use chrono::{DateTime, Datelike, TimeZone, Timelike, Utc};
use chrono_tz::America::Toronto;

use crate::astronomy::{
    coords::equatorial_to_horizontal,
    moon::moon_position,
    planets::{planet_position, Planet},
    rise_set::{jd_midnight, rise_transit_set, RiseTransitSet, H0_MOON, H0_STAR, H0_SUN},
    stars::{star_position, NAV_STARS},
    sun::sun_position,
    time::{julian_day, lst_deg},
};

// Kingston, Ontario
pub const LAT: f64 = 44.2311;
pub const LON: f64 = -76.4860; // east

#[derive(Debug, Clone, PartialEq)]
pub enum BodyType {
    Sun,
    Moon,
    Planet,
    Star,
}

#[derive(Debug, Clone)]
pub struct CelestialBody {
    pub name: String,
    pub body_type: BodyType,
    pub alt: f64,
    pub az: f64,
    pub mag: Option<f64>,
    pub rise_min: Option<f64>,    // minutes from 0h UTC
    pub transit_min: Option<f64>,
    pub set_min: Option<f64>,
    pub circumpolar: bool,
    pub never_rises: bool,
}

pub struct App {
    pub bodies: Vec<CelestialBody>,
    pub show_utc: bool,
    pub last_updated: DateTime<Utc>,
}

impl App {
    pub fn new() -> Self {
        let mut app = App {
            bodies: Vec::new(),
            show_utc: false,
            last_updated: Utc::now(),
        };
        app.update();
        app
    }

    pub fn toggle_utc(&mut self) {
        self.show_utc = !self.show_utc;
    }

    pub fn update(&mut self) {
        let now = Utc::now();
        self.last_updated = now;

        let jd = datetime_to_jd(&now);
        let lst = lst_deg(jd, LON);

        // Midnight JD for today (for rise/set)
        let jd0 = {
            let d = now.date_naive();
            jd_midnight(d.year(), d.month(), d.day())
        };

        let mut bodies: Vec<CelestialBody> = Vec::new();

        // Sun
        {
            let (ra, dec) = sun_position(jd);
            let (alt, az) = equatorial_to_horizontal(ra, dec, LAT, lst);
            let rts = rise_transit_set(ra, dec, LAT, LON, jd0, H0_SUN);
            bodies.push(body_from_rts("Sun", BodyType::Sun, alt, az, None, &rts));
        }

        // Moon
        {
            let (ra, dec) = moon_position(jd);
            let (alt, az) = equatorial_to_horizontal(ra, dec, LAT, lst);
            let rts = rise_transit_set(ra, dec, LAT, LON, jd0, H0_MOON);
            bodies.push(body_from_rts("Moon", BodyType::Moon, alt, az, None, &rts));
        }

        // Planets
        for &planet in Planet::all() {
            let (ra, dec) = planet_position(jd, planet);
            let (alt, az) = equatorial_to_horizontal(ra, dec, LAT, lst);
            let rts = rise_transit_set(ra, dec, LAT, LON, jd0, H0_STAR);
            bodies.push(body_from_rts(planet.name(), BodyType::Planet, alt, az, None, &rts));
        }

        // 57 navigational stars
        for star in NAV_STARS {
            let (ra, dec) = star_position(star, jd);
            let (alt, az) = equatorial_to_horizontal(ra, dec, LAT, lst);
            let rts = rise_transit_set(ra, dec, LAT, LON, jd0, H0_STAR);
            bodies.push(body_from_rts(
                star.name,
                BodyType::Star,
                alt, az,
                Some(star.mag),
                &rts,
            ));
        }

        // Sort: above horizon first (by altitude desc), then below (by altitude desc)
        bodies.sort_by(|a, b| b.alt.partial_cmp(&a.alt).unwrap());

        self.bodies = bodies;
    }

    /// Format a minutes-from-midnight-UTC value as HH:MM in the current display timezone.
    pub fn fmt_time(&self, min_utc: f64) -> String {
        let h = (min_utc as u32) / 60 % 24;
        let m = (min_utc as u32) % 60;
        if self.show_utc {
            format!("{:02}:{:02}z", h, m)
        } else {
            // Convert UTC minutes to local (America/Toronto)
            let base = Utc.with_ymd_and_hms(
                self.last_updated.year(),
                self.last_updated.month(),
                self.last_updated.day(),
                0, 0, 0,
            ).unwrap();
            let local_dt = base + chrono::Duration::minutes(min_utc as i64);
            let toronto = local_dt.with_timezone(&Toronto);
            format!("{:02}:{:02}", toronto.hour(), toronto.minute())
        }
    }

    pub fn header_time(&self) -> String {
        if self.show_utc {
            format!("{}", self.last_updated.format("%Y-%m-%d %H:%M:%S UTC"))
        } else {
            let local = self.last_updated.with_timezone(&Toronto);
            format!("{}", local.format("%Y-%m-%d %H:%M:%S %Z"))
        }
    }
}

fn datetime_to_jd(dt: &DateTime<Utc>) -> f64 {
    let hour = dt.hour() as f64
        + dt.minute() as f64 / 60.0
        + dt.second() as f64 / 3600.0;
    julian_day(dt.year(), dt.month(), dt.day(), hour)
}

fn body_from_rts(
    name: &str,
    body_type: BodyType,
    alt: f64,
    az: f64,
    mag: Option<f64>,
    rts: &RiseTransitSet,
) -> CelestialBody {
    CelestialBody {
        name: name.to_string(),
        body_type,
        alt,
        az,
        mag,
        rise_min: rts.rise,
        transit_min: rts.transit,
        set_min: rts.set,
        circumpolar: rts.circumpolar,
        never_rises: rts.never_rises,
    }
}
