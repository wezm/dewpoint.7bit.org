#[macro_use]
extern crate rocket;

mod config;
pub mod country;
pub mod geocoding;
pub mod home;
pub mod weather;

use std::collections::HashMap;
use std::mem;
use std::sync::Arc;

use celes::Country;
use chrono::{DateTime, Duration, Utc};
use ip2location::LocationDB as GeoDB;
use rocket::tokio::sync::Mutex;

use crate::weather::OneCall;
pub use config::DewpointConfig;

pub type CountryArray = [(&'static str, Country); 249];

pub struct Countries(pub Arc<CountryArray>);
pub struct Ip2Location(pub Arc<Mutex<GeoDB>>);

#[derive(Clone)]
pub struct WeatherCache(Arc<Mutex<HashMap<String, OneCall>>>);

trait Expires {
    const EXPIRATION_MINS: i64 = 10;

    fn stale(&self, now: DateTime<Utc>) -> bool;

    fn fresh(&self, now: DateTime<Utc>) -> bool {
        !self.stale(now)
    }
}

impl Countries {
    pub fn new() -> Self {
        // We build our own list as the long names that celes provides aren't super user-friendly
        // for the dropdown (a lot have a The prefix). The list below was generated via this
        // script: https://github.com/wezm/dotfiles/blob/master/scripts/country_list/list-countries
        // which uses the Ruby countries gem.
        let countries = [
            ("Afghanistan", Country::from_alpha2("AF").unwrap()),
            ("Åland Islands", Country::from_alpha2("AX").unwrap()),
            ("Albania", Country::from_alpha2("AL").unwrap()),
            ("Algeria", Country::from_alpha2("DZ").unwrap()),
            ("American Samoa", Country::from_alpha2("AS").unwrap()),
            ("Andorra", Country::from_alpha2("AD").unwrap()),
            ("Angola", Country::from_alpha2("AO").unwrap()),
            ("Anguilla", Country::from_alpha2("AI").unwrap()),
            ("Antarctica", Country::from_alpha2("AQ").unwrap()),
            ("Antigua and Barbuda", Country::from_alpha2("AG").unwrap()),
            ("Argentina", Country::from_alpha2("AR").unwrap()),
            ("Armenia", Country::from_alpha2("AM").unwrap()),
            ("Aruba", Country::from_alpha2("AW").unwrap()),
            ("Australia", Country::from_alpha2("AU").unwrap()),
            ("Austria", Country::from_alpha2("AT").unwrap()),
            ("Azerbaijan", Country::from_alpha2("AZ").unwrap()),
            ("Bahamas", Country::from_alpha2("BS").unwrap()),
            ("Bahrain", Country::from_alpha2("BH").unwrap()),
            ("Bangladesh", Country::from_alpha2("BD").unwrap()),
            ("Barbados", Country::from_alpha2("BB").unwrap()),
            ("Belarus", Country::from_alpha2("BY").unwrap()),
            ("Belgium", Country::from_alpha2("BE").unwrap()),
            ("Belize", Country::from_alpha2("BZ").unwrap()),
            ("Benin", Country::from_alpha2("BJ").unwrap()),
            ("Bermuda", Country::from_alpha2("BM").unwrap()),
            ("Bhutan", Country::from_alpha2("BT").unwrap()),
            ("Bolivia", Country::from_alpha2("BO").unwrap()),
            (
                "Bonaire, Sint Eustatius and Saba",
                Country::from_alpha2("BQ").unwrap(),
            ),
            (
                "Bosnia and Herzegovina",
                Country::from_alpha2("BA").unwrap(),
            ),
            ("Botswana", Country::from_alpha2("BW").unwrap()),
            ("Bouvet Island", Country::from_alpha2("BV").unwrap()),
            ("Brazil", Country::from_alpha2("BR").unwrap()),
            (
                "British Indian Ocean Territory",
                Country::from_alpha2("IO").unwrap(),
            ),
            ("Brunei Darussalam", Country::from_alpha2("BN").unwrap()),
            ("Bulgaria", Country::from_alpha2("BG").unwrap()),
            ("Burkina Faso", Country::from_alpha2("BF").unwrap()),
            ("Burundi", Country::from_alpha2("BI").unwrap()),
            ("Cabo Verde", Country::from_alpha2("CV").unwrap()),
            ("Cambodia", Country::from_alpha2("KH").unwrap()),
            ("Cameroon", Country::from_alpha2("CM").unwrap()),
            ("Canada", Country::from_alpha2("CA").unwrap()),
            ("Cayman Islands", Country::from_alpha2("KY").unwrap()),
            (
                "Central African Republic",
                Country::from_alpha2("CF").unwrap(),
            ),
            ("Chad", Country::from_alpha2("TD").unwrap()),
            ("Chile", Country::from_alpha2("CL").unwrap()),
            ("China", Country::from_alpha2("CN").unwrap()),
            ("Christmas Island", Country::from_alpha2("CX").unwrap()),
            (
                "Cocos (Keeling) Islands",
                Country::from_alpha2("CC").unwrap(),
            ),
            ("Colombia", Country::from_alpha2("CO").unwrap()),
            ("Comoros", Country::from_alpha2("KM").unwrap()),
            ("Congo", Country::from_alpha2("CG").unwrap()),
            (
                "Congo, The Democratic Republic of the",
                Country::from_alpha2("CD").unwrap(),
            ),
            ("Cook Islands", Country::from_alpha2("CK").unwrap()),
            ("Costa Rica", Country::from_alpha2("CR").unwrap()),
            ("Côte d'Ivoire", Country::from_alpha2("CI").unwrap()),
            ("Croatia", Country::from_alpha2("HR").unwrap()),
            ("Cuba", Country::from_alpha2("CU").unwrap()),
            ("Curaçao", Country::from_alpha2("CW").unwrap()),
            ("Cyprus", Country::from_alpha2("CY").unwrap()),
            ("Czechia", Country::from_alpha2("CZ").unwrap()),
            ("Denmark", Country::from_alpha2("DK").unwrap()),
            ("Djibouti", Country::from_alpha2("DJ").unwrap()),
            ("Dominica", Country::from_alpha2("DM").unwrap()),
            ("Dominican Republic", Country::from_alpha2("DO").unwrap()),
            ("Ecuador", Country::from_alpha2("EC").unwrap()),
            ("Egypt", Country::from_alpha2("EG").unwrap()),
            ("El Salvador", Country::from_alpha2("SV").unwrap()),
            ("Equatorial Guinea", Country::from_alpha2("GQ").unwrap()),
            ("Eritrea", Country::from_alpha2("ER").unwrap()),
            ("Estonia", Country::from_alpha2("EE").unwrap()),
            ("Eswatini", Country::from_alpha2("SZ").unwrap()),
            ("Ethiopia", Country::from_alpha2("ET").unwrap()),
            (
                "Falkland Islands (Malvinas)",
                Country::from_alpha2("FK").unwrap(),
            ),
            ("Faroe Islands", Country::from_alpha2("FO").unwrap()),
            ("Fiji", Country::from_alpha2("FJ").unwrap()),
            ("Finland", Country::from_alpha2("FI").unwrap()),
            ("France", Country::from_alpha2("FR").unwrap()),
            ("French Guiana", Country::from_alpha2("GF").unwrap()),
            ("French Polynesia", Country::from_alpha2("PF").unwrap()),
            (
                "French Southern Territories",
                Country::from_alpha2("TF").unwrap(),
            ),
            ("Gabon", Country::from_alpha2("GA").unwrap()),
            ("Gambia", Country::from_alpha2("GM").unwrap()),
            ("Georgia", Country::from_alpha2("GE").unwrap()),
            ("Germany", Country::from_alpha2("DE").unwrap()),
            ("Ghana", Country::from_alpha2("GH").unwrap()),
            ("Gibraltar", Country::from_alpha2("GI").unwrap()),
            ("Greece", Country::from_alpha2("GR").unwrap()),
            ("Greenland", Country::from_alpha2("GL").unwrap()),
            ("Grenada", Country::from_alpha2("GD").unwrap()),
            ("Guadeloupe", Country::from_alpha2("GP").unwrap()),
            ("Guam", Country::from_alpha2("GU").unwrap()),
            ("Guatemala", Country::from_alpha2("GT").unwrap()),
            ("Guernsey", Country::from_alpha2("GG").unwrap()),
            ("Guinea", Country::from_alpha2("GN").unwrap()),
            ("Guinea-Bissau", Country::from_alpha2("GW").unwrap()),
            ("Guyana", Country::from_alpha2("GY").unwrap()),
            ("Haiti", Country::from_alpha2("HT").unwrap()),
            (
                "Heard Island and McDonald Islands",
                Country::from_alpha2("HM").unwrap(),
            ),
            (
                "Holy See (Vatican City State)",
                Country::from_alpha2("VA").unwrap(),
            ),
            ("Honduras", Country::from_alpha2("HN").unwrap()),
            ("Hong Kong", Country::from_alpha2("HK").unwrap()),
            ("Hungary", Country::from_alpha2("HU").unwrap()),
            ("Iceland", Country::from_alpha2("IS").unwrap()),
            ("India", Country::from_alpha2("IN").unwrap()),
            ("Indonesia", Country::from_alpha2("ID").unwrap()),
            (
                "Iran, Islamic Republic of",
                Country::from_alpha2("IR").unwrap(),
            ),
            ("Iraq", Country::from_alpha2("IQ").unwrap()),
            ("Ireland", Country::from_alpha2("IE").unwrap()),
            ("Isle of Man", Country::from_alpha2("IM").unwrap()),
            ("Israel", Country::from_alpha2("IL").unwrap()),
            ("Italy", Country::from_alpha2("IT").unwrap()),
            ("Jamaica", Country::from_alpha2("JM").unwrap()),
            ("Japan", Country::from_alpha2("JP").unwrap()),
            ("Jersey", Country::from_alpha2("JE").unwrap()),
            ("Jordan", Country::from_alpha2("JO").unwrap()),
            ("Kazakhstan", Country::from_alpha2("KZ").unwrap()),
            ("Kenya", Country::from_alpha2("KE").unwrap()),
            ("Kiribati", Country::from_alpha2("KI").unwrap()),
            (
                "Korea, Democratic People's Republic of",
                Country::from_alpha2("KP").unwrap(),
            ),
            ("Korea, Republic of", Country::from_alpha2("KR").unwrap()),
            ("Kuwait", Country::from_alpha2("KW").unwrap()),
            ("Kyrgyzstan", Country::from_alpha2("KG").unwrap()),
            (
                "Lao People's Democratic Republic",
                Country::from_alpha2("LA").unwrap(),
            ),
            ("Latvia", Country::from_alpha2("LV").unwrap()),
            ("Lebanon", Country::from_alpha2("LB").unwrap()),
            ("Lesotho", Country::from_alpha2("LS").unwrap()),
            ("Liberia", Country::from_alpha2("LR").unwrap()),
            ("Libya", Country::from_alpha2("LY").unwrap()),
            ("Liechtenstein", Country::from_alpha2("LI").unwrap()),
            ("Lithuania", Country::from_alpha2("LT").unwrap()),
            ("Luxembourg", Country::from_alpha2("LU").unwrap()),
            ("Macao", Country::from_alpha2("MO").unwrap()),
            ("Madagascar", Country::from_alpha2("MG").unwrap()),
            ("Malawi", Country::from_alpha2("MW").unwrap()),
            ("Malaysia", Country::from_alpha2("MY").unwrap()),
            ("Maldives", Country::from_alpha2("MV").unwrap()),
            ("Mali", Country::from_alpha2("ML").unwrap()),
            ("Malta", Country::from_alpha2("MT").unwrap()),
            ("Marshall Islands", Country::from_alpha2("MH").unwrap()),
            ("Martinique", Country::from_alpha2("MQ").unwrap()),
            ("Mauritania", Country::from_alpha2("MR").unwrap()),
            ("Mauritius", Country::from_alpha2("MU").unwrap()),
            ("Mayotte", Country::from_alpha2("YT").unwrap()),
            ("Mexico", Country::from_alpha2("MX").unwrap()),
            (
                "Micronesia, Federated States of",
                Country::from_alpha2("FM").unwrap(),
            ),
            ("Moldova", Country::from_alpha2("MD").unwrap()),
            ("Monaco", Country::from_alpha2("MC").unwrap()),
            ("Mongolia", Country::from_alpha2("MN").unwrap()),
            ("Montenegro", Country::from_alpha2("ME").unwrap()),
            ("Montserrat", Country::from_alpha2("MS").unwrap()),
            ("Morocco", Country::from_alpha2("MA").unwrap()),
            ("Mozambique", Country::from_alpha2("MZ").unwrap()),
            ("Myanmar", Country::from_alpha2("MM").unwrap()),
            ("Namibia", Country::from_alpha2("NA").unwrap()),
            ("Nauru", Country::from_alpha2("NR").unwrap()),
            ("Nepal", Country::from_alpha2("NP").unwrap()),
            ("Netherlands", Country::from_alpha2("NL").unwrap()),
            ("New Caledonia", Country::from_alpha2("NC").unwrap()),
            ("New Zealand", Country::from_alpha2("NZ").unwrap()),
            ("Nicaragua", Country::from_alpha2("NI").unwrap()),
            ("Niger", Country::from_alpha2("NE").unwrap()),
            ("Nigeria", Country::from_alpha2("NG").unwrap()),
            ("Niue", Country::from_alpha2("NU").unwrap()),
            ("Norfolk Island", Country::from_alpha2("NF").unwrap()),
            ("North Macedonia", Country::from_alpha2("MK").unwrap()),
            (
                "Northern Mariana Islands",
                Country::from_alpha2("MP").unwrap(),
            ),
            ("Norway", Country::from_alpha2("NO").unwrap()),
            ("Oman", Country::from_alpha2("OM").unwrap()),
            ("Pakistan", Country::from_alpha2("PK").unwrap()),
            ("Palau", Country::from_alpha2("PW").unwrap()),
            ("Palestine, State of", Country::from_alpha2("PS").unwrap()),
            ("Panama", Country::from_alpha2("PA").unwrap()),
            ("Papua New Guinea", Country::from_alpha2("PG").unwrap()),
            ("Paraguay", Country::from_alpha2("PY").unwrap()),
            ("Peru", Country::from_alpha2("PE").unwrap()),
            ("Philippines", Country::from_alpha2("PH").unwrap()),
            ("Pitcairn", Country::from_alpha2("PN").unwrap()),
            ("Poland", Country::from_alpha2("PL").unwrap()),
            ("Portugal", Country::from_alpha2("PT").unwrap()),
            ("Puerto Rico", Country::from_alpha2("PR").unwrap()),
            ("Qatar", Country::from_alpha2("QA").unwrap()),
            ("Réunion", Country::from_alpha2("RE").unwrap()),
            ("Romania", Country::from_alpha2("RO").unwrap()),
            ("Russian Federation", Country::from_alpha2("RU").unwrap()),
            ("Rwanda", Country::from_alpha2("RW").unwrap()),
            ("Saint Barthélemy", Country::from_alpha2("BL").unwrap()),
            (
                "Saint Helena, Ascension and Tristan da Cunha",
                Country::from_alpha2("SH").unwrap(),
            ),
            ("Saint Kitts and Nevis", Country::from_alpha2("KN").unwrap()),
            ("Saint Lucia", Country::from_alpha2("LC").unwrap()),
            (
                "Saint Martin (French part)",
                Country::from_alpha2("MF").unwrap(),
            ),
            (
                "Saint Pierre and Miquelon",
                Country::from_alpha2("PM").unwrap(),
            ),
            (
                "Saint Vincent and the Grenadines",
                Country::from_alpha2("VC").unwrap(),
            ),
            ("Samoa", Country::from_alpha2("WS").unwrap()),
            ("San Marino", Country::from_alpha2("SM").unwrap()),
            ("Sao Tome and Principe", Country::from_alpha2("ST").unwrap()),
            ("Saudi Arabia", Country::from_alpha2("SA").unwrap()),
            ("Senegal", Country::from_alpha2("SN").unwrap()),
            ("Serbia", Country::from_alpha2("RS").unwrap()),
            ("Seychelles", Country::from_alpha2("SC").unwrap()),
            ("Sierra Leone", Country::from_alpha2("SL").unwrap()),
            ("Singapore", Country::from_alpha2("SG").unwrap()),
            (
                "Sint Maarten (Dutch part)",
                Country::from_alpha2("SX").unwrap(),
            ),
            ("Slovakia", Country::from_alpha2("SK").unwrap()),
            ("Slovenia", Country::from_alpha2("SI").unwrap()),
            ("Solomon Islands", Country::from_alpha2("SB").unwrap()),
            ("Somalia", Country::from_alpha2("SO").unwrap()),
            ("South Africa", Country::from_alpha2("ZA").unwrap()),
            (
                "South Georgia and the South Sandwich Islands",
                Country::from_alpha2("GS").unwrap(),
            ),
            ("South Sudan", Country::from_alpha2("SS").unwrap()),
            ("Spain", Country::from_alpha2("ES").unwrap()),
            ("Sri Lanka", Country::from_alpha2("LK").unwrap()),
            ("Sudan", Country::from_alpha2("SD").unwrap()),
            ("Suriname", Country::from_alpha2("SR").unwrap()),
            (
                "Svalbard and Jan Mayen",
                Country::from_alpha2("SJ").unwrap(),
            ),
            ("Sweden", Country::from_alpha2("SE").unwrap()),
            ("Switzerland", Country::from_alpha2("CH").unwrap()),
            ("Syrian Arab Republic", Country::from_alpha2("SY").unwrap()),
            ("Taiwan", Country::from_alpha2("TW").unwrap()),
            ("Tajikistan", Country::from_alpha2("TJ").unwrap()),
            ("Tanzania", Country::from_alpha2("TZ").unwrap()),
            ("Thailand", Country::from_alpha2("TH").unwrap()),
            ("Timor-Leste", Country::from_alpha2("TL").unwrap()),
            ("Togo", Country::from_alpha2("TG").unwrap()),
            ("Tokelau", Country::from_alpha2("TK").unwrap()),
            ("Tonga", Country::from_alpha2("TO").unwrap()),
            ("Trinidad and Tobago", Country::from_alpha2("TT").unwrap()),
            ("Tunisia", Country::from_alpha2("TN").unwrap()),
            ("Turkey", Country::from_alpha2("TR").unwrap()),
            ("Turkmenistan", Country::from_alpha2("TM").unwrap()),
            (
                "Turks and Caicos Islands",
                Country::from_alpha2("TC").unwrap(),
            ),
            ("Tuvalu", Country::from_alpha2("TV").unwrap()),
            ("Uganda", Country::from_alpha2("UG").unwrap()),
            ("Ukraine", Country::from_alpha2("UA").unwrap()),
            ("United Arab Emirates", Country::from_alpha2("AE").unwrap()),
            ("United Kingdom", Country::from_alpha2("GB").unwrap()),
            (
                "United States Minor Outlying Islands",
                Country::from_alpha2("UM").unwrap(),
            ),
            ("United States", Country::from_alpha2("US").unwrap()),
            ("Uruguay", Country::from_alpha2("UY").unwrap()),
            ("Uzbekistan", Country::from_alpha2("UZ").unwrap()),
            ("Vanuatu", Country::from_alpha2("VU").unwrap()),
            ("Venezuela", Country::from_alpha2("VE").unwrap()),
            ("Vietnam", Country::from_alpha2("VN").unwrap()),
            (
                "Virgin Islands, British",
                Country::from_alpha2("VG").unwrap(),
            ),
            ("Virgin Islands, U.S.", Country::from_alpha2("VI").unwrap()),
            ("Wallis and Futuna", Country::from_alpha2("WF").unwrap()),
            ("Western Sahara", Country::from_alpha2("EH").unwrap()),
            ("Yemen", Country::from_alpha2("YE").unwrap()),
            ("Zambia", Country::from_alpha2("ZM").unwrap()),
            ("Zimbabwe", Country::from_alpha2("ZW").unwrap()),
        ];
        Countries(Arc::new(countries))
    }
}

impl Ip2Location {
    pub fn new(geodb: GeoDB) -> Self {
        Ip2Location(Arc::new(Mutex::new(geodb)))
    }
}

impl WeatherCache {
    pub fn new() -> Self {
        WeatherCache(Arc::new(Mutex::new(HashMap::new())))
    }

    /// Purge expired entries
    pub async fn clean(&self) {
        let mut locked = self.0.lock().await;

        // Move the current map out of the Mutex to filter and replace it
        let cache = mem::replace(&mut *locked, HashMap::new());

        let now = Utc::now();
        *locked = cache
            .into_iter()
            .filter(|(_url, data)| data.fresh(now))
            .collect();
    }

    pub async fn get_or_fetch(&self, url: String) -> Result<OneCall, reqwest::Error> {
        let mut cache = self.0.lock().await;
        match cache.get(&url) {
            Some(data) => {
                let now = Utc::now();
                if data.stale(now) {
                    info!("Weather cache hit: stale");
                    // Stale, refresh cache
                    let data = Self::fetch(&url).await?;
                    cache.insert(url, data.clone());
                    Ok(data)
                } else {
                    info!("Weather cache hit: fresh");
                    // Fresh enough
                    Ok(data.to_owned())
                }
            }
            None => {
                info!("Weather cache miss");
                let data = Self::fetch(&url).await?;
                cache.insert(url, data.clone());
                Ok(data)
            }
        }
    }

    async fn fetch(url: &str) -> Result<OneCall, reqwest::Error> {
        reqwest::get(url).await?.json().await
    }
}

impl Expires for OneCall {
    fn stale(&self, now: DateTime<Utc>) -> bool {
        (now - self.current.dt.to_chrono()) > Duration::minutes(Self::EXPIRATION_MINS)
    }
}
