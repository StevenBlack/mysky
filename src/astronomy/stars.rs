use super::time::{j2000_centuries, reduce_deg};

#[derive(Debug, Clone)]
pub struct NavStar {
    pub name: &'static str,
    pub ra_h: f64,  // J2000 RA in decimal hours
    pub dec_d: f64, // J2000 Dec in degrees
    pub mag: f64,
}

impl NavStar {
    fn ra_deg(&self) -> f64 {
        self.ra_h * 15.0
    }
}

/// The 57 navigational stars of the Nautical Almanac (J2000 coordinates).
pub const NAV_STARS: &[NavStar] = &[
    NavStar { name: "Achernar",        ra_h:  1.6285833, dec_d: -57.2366667, mag:  0.46 },
    NavStar { name: "Acrux",           ra_h: 12.4432917, dec_d: -63.0993333, mag:  0.87 },
    NavStar { name: "Adhara",          ra_h:  6.9770833, dec_d: -28.9720556, mag:  1.50 },
    NavStar { name: "Aldebaran",       ra_h:  4.5986667, dec_d:  16.5092222, mag:  0.85 },
    NavStar { name: "Alioth",          ra_h: 12.9004722, dec_d:  55.9597222, mag:  1.77 },
    NavStar { name: "Alkaid",          ra_h: 13.7923333, dec_d:  49.3133333, mag:  1.86 },
    NavStar { name: "Al Na'ir",        ra_h: 22.1371944, dec_d: -46.9610000, mag:  1.74 },
    NavStar { name: "Alnilam",         ra_h:  5.6035556, dec_d:  -1.2019444, mag:  1.70 },
    NavStar { name: "Alphard",         ra_h:  9.4597778, dec_d:  -8.6586111, mag:  1.98 },
    NavStar { name: "Alphecca",        ra_h: 15.5781389, dec_d:  26.7147222, mag:  2.23 },
    NavStar { name: "Alpheratz",       ra_h:  0.1397778, dec_d:  29.0905000, mag:  2.06 },
    NavStar { name: "Altair",          ra_h: 19.8463889, dec_d:   8.8683333, mag:  0.77 },
    NavStar { name: "Ankaa",           ra_h:  0.4380278, dec_d: -42.3061111, mag:  2.40 },
    NavStar { name: "Antares",         ra_h: 16.4901389, dec_d: -26.4319444, mag:  1.06 },
    NavStar { name: "Arcturus",        ra_h: 14.2610833, dec_d:  19.1827778, mag: -0.05 },
    NavStar { name: "Atria",           ra_h: 16.8111111, dec_d: -69.0277778, mag:  1.92 },
    NavStar { name: "Avior",           ra_h:  8.3752222, dec_d: -59.5094444, mag:  1.86 },
    NavStar { name: "Bellatrix",       ra_h:  5.4188611, dec_d:   6.3497222, mag:  1.64 },
    NavStar { name: "Betelgeuse",      ra_h:  5.9195278, dec_d:   7.4069444, mag:  0.58 },
    NavStar { name: "Canopus",         ra_h:  6.3991944, dec_d: -52.6958333, mag: -0.62 },
    NavStar { name: "Capella",         ra_h:  5.2781667, dec_d:  45.9980556, mag:  0.08 },
    NavStar { name: "Deneb",           ra_h: 20.6905278, dec_d:  45.2802778, mag:  1.25 },
    NavStar { name: "Denebola",        ra_h: 11.8176667, dec_d:  14.5719444, mag:  2.14 },
    NavStar { name: "Diphda",          ra_h:  0.7265278, dec_d: -17.9866667, mag:  2.04 },
    NavStar { name: "Dubhe",           ra_h: 11.0621389, dec_d:  61.7511111, mag:  1.81 },
    NavStar { name: "Elnath",          ra_h:  5.4381944, dec_d:  28.6075000, mag:  1.65 },
    NavStar { name: "Eltanin",         ra_h: 17.9434444, dec_d:  51.4888889, mag:  2.24 },
    NavStar { name: "Enif",            ra_h: 21.7364167, dec_d:   9.8750000, mag:  2.38 },
    NavStar { name: "Fomalhaut",       ra_h: 22.9608333, dec_d: -29.6222222, mag:  1.16 },
    NavStar { name: "Gacrux",          ra_h: 12.5191667, dec_d: -57.1130556, mag:  1.63 },
    NavStar { name: "Gienah",          ra_h: 12.2634444, dec_d: -17.5419444, mag:  2.59 },
    NavStar { name: "Hadar",           ra_h: 14.0637222, dec_d: -60.3730556, mag:  0.61 },
    NavStar { name: "Hamal",           ra_h:  2.1195556, dec_d:  23.4625000, mag:  2.01 },
    NavStar { name: "Kaus Australis",  ra_h: 18.4028611, dec_d: -34.3847222, mag:  1.85 },
    NavStar { name: "Kochab",          ra_h: 14.8451000, dec_d:  74.1555556, mag:  2.08 },
    NavStar { name: "Markab",          ra_h: 23.0793611, dec_d:  15.2052778, mag:  2.49 },
    NavStar { name: "Menkar",          ra_h:  3.0380000, dec_d:   4.0897222, mag:  2.54 },
    NavStar { name: "Menkent",         ra_h: 14.1113611, dec_d: -36.3700000, mag:  2.06 },
    NavStar { name: "Miaplacidus",     ra_h:  9.2199722, dec_d: -69.7172222, mag:  1.68 },
    NavStar { name: "Mimosa",          ra_h: 12.7953611, dec_d: -59.6888889, mag:  1.25 },
    NavStar { name: "Mirfak",          ra_h:  3.4053889, dec_d:  49.8611111, mag:  1.79 },
    NavStar { name: "Nunki",           ra_h: 18.9210833, dec_d: -26.2966667, mag:  2.05 },
    NavStar { name: "Peacock",         ra_h: 20.4274722, dec_d: -56.7350000, mag:  1.94 },
    // Polaris is not a navigational star but it is useful, so including it here.
    NavStar { name: "Polaris",         ra_h: 37.9545,    dec_d:  89.2641,    mag:  1.98 },
    // Castor is not a navigational star (it's a triple binary star system) but including it here.
    NavStar { name: "Castor",          ra_h:  7.576634,  dec_d:  31.888276,  mag:  1.58 },
    NavStar { name: "Pollux",          ra_h:  7.7552778, dec_d:  28.0261111, mag:  1.14 },
    NavStar { name: "Procyon",         ra_h:  7.6550278, dec_d:   5.2250000, mag:  0.38 },
    NavStar { name: "Rasalhague",      ra_h: 17.5822500, dec_d:  12.5600000, mag:  2.08 },
    NavStar { name: "Regulus",         ra_h: 10.1395278, dec_d:  11.9672222, mag:  1.36 },
    NavStar { name: "Rigel",           ra_h:  5.2422917, dec_d:  -8.2016667, mag:  0.18 },
    NavStar { name: "Rigil Kentaurus", ra_h: 14.6601389, dec_d: -60.8338889, mag: -0.01 },
    NavStar { name: "Sabik",           ra_h: 17.1729722, dec_d: -15.7250000, mag:  2.43 },
    NavStar { name: "Schedar",         ra_h:  0.6751111, dec_d:  56.5372222, mag:  2.24 },
    NavStar { name: "Shaula",          ra_h: 17.5601389, dec_d: -37.1038889, mag:  1.62 },
    NavStar { name: "Sirius",          ra_h:  6.7524722, dec_d: -16.7161111, mag: -1.44 },
    NavStar { name: "Spica",           ra_h: 13.4198889, dec_d: -11.1613889, mag:  1.05 },
    NavStar { name: "Suhail",          ra_h:  9.1332778, dec_d: -43.4325000, mag:  2.23 },
    NavStar { name: "Vega",            ra_h: 18.6156389, dec_d:  38.7836111, mag:  0.03 },
    NavStar { name: "Zubenelgenubi",   ra_h: 14.8479444, dec_d: -16.0416667, mag:  2.75 },
];

/// Apply IAU 1976 precession to get current RA/Dec from J2000 coords.
/// Returns (RA degrees, Dec degrees) at the epoch of JD.
pub fn star_position(star: &NavStar, jd: f64) -> (f64, f64) {
    let t = j2000_centuries(jd);
    let ra0 = star.ra_deg().to_radians();
    let dec0 = star.dec_d.to_radians();

    // General precession constants (Lieske 1979, degrees)
    let m = 1.2812323 * t + 0.0003879 * t * t + 0.0000101 * t * t * t;
    let n = 0.5567530 * t - 0.0001185 * t * t - 0.0000116 * t * t * t;

    let m_r = m.to_radians();
    let n_r = n.to_radians();

    let delta_ra = m_r + n_r * ra0.sin() * dec0.tan();
    let delta_dec = n_r * ra0.cos();

    let ra = reduce_deg((ra0 + delta_ra).to_degrees());
    let dec = (dec0 + delta_dec).to_degrees();

    (ra, dec)
}
