pub mod scrape;
pub mod pull;

use serde::{Deserialize, Serialize};

use scrape::wikipedia::*;
use pull::datahub::*;

pub const WIKIPEDIA_COUNTRIES_URL: &str = "https://en.wikipedia.org/wiki/List_of_ISO_3166_country_codes";
pub const DATAHUB_COUNTRIES_URL: &str = "https://www.datahub.io/core/country-list/r/data.json";
pub const WIKIPEDIA_CURRENCIES_URL: &str = "https://en.wikipedia.org/wiki/List_of_circulating_currencies";
pub const DATAHUB_CURRENCIES_URL: &str = "https://datahub.io/core/currency-codes/r/codes-all.json";
pub const WIKIPEDIA_FLAG_EMOJIS_URL: &str = "https://en.wikipedia.org/wiki/Regional_indicator_symbol";
pub const WIKIPEDIA_CALLING_CODES_URL: &str = "https://en.wikipedia.org/wiki/List_of_country_calling_codes";
pub const WIKIPEDIA_ENDONYMS_URL: &str = "https://en.wikipedia.org/wiki/List_of_countries_and_dependencies_and_their_capitals_in_native_languages";

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Currency {
    pub name: String,
    pub iso_4217: Option<String>,
    pub symbol: Option<String>,
    pub fraction: Option<String>,
    pub fractions_in_unit: Option<i32>,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Time {
    pub zone: String,
    pub dst: Option<String>,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct Country {
    pub exonym: String,
    pub name: String,
    pub capital: Option<String>,
    pub iso_3166_1_a3: Option<String>,
    pub tld: Option<String>,
    pub flag: Option<String>,
    pub currency: Option<Currency>,
    pub time: Option<Time>,
    pub calling_code: Option<String>,
    pub endonyms: Option<Vec<String>>,
    pub languages: Option<Vec<String>>,
}

impl Country {
    pub fn from_wikipedia(item: WikipediaScrapedCountry) -> Self {
        Self {
            exonym: item.exonym, name: item.name, capital: None,
            iso_3166_1_a3: Some(item.iso_3166_1_a3),
            tld: match item.tld.starts_with(".") {
                true => Some(item.tld),
                false => None,
            }, flag: None, currency: None,
            time: None, calling_code: None, endonyms: None,
            languages: None
        }
    }
    pub fn from_country_list(item: DatahubCountry) -> Self {
        Self {
            exonym: item.name.to_owned(), name: item.name, capital: None,
            iso_3166_1_a3: None, tld: None, flag: None, currency: None,
            time: None, calling_code: None, endonyms: None,
            languages: None
        }
    }
    pub fn add_wikipedia_currency(&mut self, item: WikipediaScrapedCurrency) {
        let mut currency = match self.currency {
            Some(ref c) => c.to_owned(),
            None => Currency {
                name: item.name, iso_4217: item.iso_4217.to_owned(), symbol:
                item.symbol.to_owned(), fraction: item.fraction.to_owned(),
                fractions_in_unit: item.fractions_in_unit.to_owned()
            }
        };

        if currency.iso_4217.is_none() {
            currency.iso_4217 = item.iso_4217;
        }

        if currency.symbol.is_none() {
            currency.symbol = item.symbol;
        }

        if currency.fraction.is_none() {
            currency.fraction = item.fraction;
        }

        if currency.fractions_in_unit.is_none() {
            currency.fractions_in_unit = item.fractions_in_unit;
        }

        self.currency = Some(currency)
    }
    pub fn add_currency_from_list(&mut self, item: DatahubCurrency) {
        let mut currency = match self.currency {
            Some(ref c) => c.to_owned(),
            None => match item.iso_4217.is_some() && item.name.is_some() {
                true => Currency {
                    name: item.name.unwrap(),
                    iso_4217: Some(item.iso_4217.to_owned().unwrap()),
                    ..Currency::default()
                },
                false => return,
            }
        };

        if currency.iso_4217.is_none() && item.iso_4217.is_some() {
            currency.iso_4217 = Some(item.iso_4217.unwrap());
        }

        self.currency = Some(currency)
    }
    pub fn add_wikipedia_cc_tz(&mut self, item: WikipediaScrapedCcTz) {
        self.time = Some(Time { zone: item.tz, dst: item.dst });
        self.calling_code = Some(item.code);
    }
    pub fn add_wikipedia_endonyms_langs(&mut self, item: WikipediaScrapedEndonyms) {
        if self.capital.is_none() {
            self.capital = Some(item.capital);
        }

        let mut endonyms = item.endonyms.into_iter()
            .filter(|s|s.ne(&self.exonym))
            .collect::<Vec<String>>();

        if let Some(ref v) = self.endonyms {
            endonyms.extend(v.to_owned());
        }

        endonyms.sort();
        endonyms.dedup();

        if ! endonyms.is_empty() {
            self.endonyms = Some(endonyms);
        }

        let mut langs = item.languages.to_owned();

        if let Some(ref v) = self.languages {
            langs.extend(v.to_owned());
        }

        langs.sort();
        langs.dedup();

        if ! langs.is_empty() {
            self.languages = Some(langs);
        }
    }
}
