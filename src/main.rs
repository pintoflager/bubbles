use std::collections::BTreeMap;
use std::fs::write;
use std::path::PathBuf;

use bubbles_lib::*;
use bubbles_lib::pull::datahub::*;
use bubbles_lib::scrape::wikipedia::*;

fn main() {
    let mut base = match WikipediaScrapedCountry::scrape(WIKIPEDIA_COUNTRIES_URL) {
        Ok(v) => v.into_iter()
            .map(|i|
                (i.iso_3166_1_a2.to_owned(), Country::from_wikipedia(i))
            )
            .collect::<BTreeMap<String, Country>>(),
        Err(e) => {    
            eprintln!("Failed to build base data from wikipedia: {}", e);

            BTreeMap::new()
        },
    };

    // We either have the base data or not. Start extending it
    match DatahubCountry::pull(DATAHUB_COUNTRIES_URL) {
        Ok(v) => {
            for i in v {
                match base.get_mut(&i.code) {
                    Some(c) => {
                        // The name field would be great if it would be written as
                        // the short english exonym as the page itself (wikipedia..)
                        // but no.
                        // This list might have english exonyms instead.
                        if c.exonym.is_empty() || c.exonym.ne(&i.name) {
                            let mut endonyms = match c.endonyms {
                                Some(ref v) => v.to_owned(),
                                None => vec![],
                            };

                            // Take the overwritten value into the endonyms
                            if ! endonyms.contains(&c.exonym) {
                                endonyms.push(c.exonym.to_owned());
                                c.endonyms = Some(endonyms);
                            }
                            
                            c.exonym = i.name.to_owned();
                        }

                        if c.name.is_empty() {
                            c.name = i.name;
                        }
                    },
                    None => {
                        base.insert(
                            i.code.to_owned(),
                            Country::from_country_list(i)
                        );
                    }
                }
            }
        },
        Err(e) => eprintln!("Failed to pull country data from datahub: {}", e),
    };

    // Since many datasets refer countries by their 'names' and not by their ISO codes
    // or their exonyms more names we have the merrier.
    match WikipediaScrapedEndonyms::scrape(WIKIPEDIA_ENDONYMS_URL) {
        Ok(v) => {
            for c in v {
                match base.iter_mut().find(|(_, b)|b.exonym.to_lowercase().eq(&c.exonym.to_lowercase())) {
                    Some((_, i)) => i.add_wikipedia_endonyms_langs(c),
                    None => match endonym_match(&mut base, c.exonym.to_string()) {
                        Some(i) => i.add_wikipedia_endonyms_langs(c),
                        None => eprintln!("Skip adding endonyms {:?} and languages {:?} \
                        to unknown country {}", c.endonyms, c.languages, c.exonym),
                    }
                }
            }
        },
        Err(e) => eprintln!("Failed to add endonyms data from wikipedia: {}", e),
    }

    if base.len() == 0 {
        panic!("Well that went well. Not even one country was found.")
    }

    // Try to take currencies first from this list which has at least some of the
    // names in their short form
    match DatahubCurrency::pull(DATAHUB_CURRENCIES_URL) {
        Ok(v) => {
            for c in v {
                match base.iter_mut().find(|(_, b)|b.exonym.to_lowercase().eq(&c.exonym.to_lowercase())) {
                    Some((_, i)) => i.add_currency_from_list(c),
                    None => match endonym_match(&mut base, c.exonym.to_string()) {
                        Some(i) => i.add_currency_from_list(c),
                        None => eprintln!("Skip adding currency {:?} to unknown country {}",
                            c.name, c.exonym),
                    }
                }
            }
        },
        Err(e) => eprintln!("Failed to pull currency data from datahub: {}", e),
    }

    // Add currencies if we can a. pull the source data b. have existing country item
    // to match against
    match WikipediaScrapedCurrency::scrape(WIKIPEDIA_CURRENCIES_URL) {
        Ok(v) => {
            for c in v {
                match base.iter_mut().find(|(_, b)|b.exonym.to_lowercase().eq(&c.exonym.to_lowercase())) {
                    Some((_, i)) => i.add_wikipedia_currency(c),
                    None => match endonym_match(&mut base, c.exonym.to_string()) {
                        Some(i) => i.add_wikipedia_currency(c),
                        None => eprintln!("Skip adding currency {} to unknown country {}",
                            c.name, c.exonym),
                    }
                }
            }
        },
        Err(e) => eprintln!("Failed to add currency data from wikipedia: {}", e),
    }

    // Add flag emojis scraping wikipedia again
    match WikipediaScrapedFlag::scrape(WIKIPEDIA_FLAG_EMOJIS_URL) {
        Ok(v) => {
            for i in v {
                match base.get_mut(&i.iso_3166_1_a2) {
                    Some(c) => { c.flag = Some(i.flag); },
                    None => {
                        eprintln!("Skip adding flag {} to unknown country {}",
                            i.flag, i.iso_3166_1_a2);
                    }
                }
            }
        },
        Err(e) => eprintln!("Failed to add emoji flag data from wikipedia: {}", e),
    }

    match WikipediaScrapedCcTz::scrape(WIKIPEDIA_CALLING_CODES_URL) {
        Ok(v) => {
            for c in v {
                match base.iter_mut().find(|(_, b)|b.exonym.to_lowercase().eq(&c.exonym.to_lowercase())) {
                    Some((_, i)) => i.add_wikipedia_cc_tz(c),
                    None => match endonym_match(&mut base, c.exonym.to_string()) {
                        Some(i) => i.add_wikipedia_cc_tz(c),
                        None => eprintln!("Skip adding calling code {} and timezone {} \
                            to unknown country {}", c.code, c.tz, c.exonym),
                    }
                }
            }
        },
        Err(e) => eprintln!("Failed to add calling code and timezone data \
            from wikipedia: {}", e),
    }

    let file = PathBuf::from("./out.json");
    let json = serde_json::to_string_pretty(&base)
        .expect("Failed to read countries data to JSON string");

    write(&file, json.as_bytes())
        .expect("Failed to write countries json to string");
}

fn endonym_match<'a>(base: &'a mut BTreeMap<String, Country>, exonym: String) -> Option<&'a mut Country> {
    base.values_mut().find(|b|
        b.endonyms.is_some() && b.endonyms.to_owned().unwrap().into_iter()
        .map(|s|s.to_lowercase()).collect::<Vec<String>>()
        .contains(&exonym.to_lowercase())
    )
}