use anyhow::{Result, bail};
use scraper::{ElementRef, Selector};
use serde::Deserialize;


#[derive(Deserialize, Default, Debug)]
pub struct WikipediaScrapedCountry {
    pub exonym: String,
    pub name: String,
    pub iso_3166_1_a2: String,
    pub iso_3166_1_a3: String,
    pub tld: String
}

impl WikipediaScrapedCountry {
    pub fn scrape(url: &str) -> Result<Vec<Self>> {
        println!("Scrape ISO 3166-1 country data from wikipedia...");
    
        // Define takes and discards
        let columns = 8;
        let discard_col_indexes = vec![2, 5, 6];
    
        let table = match super::table_scraper(url, columns, discard_col_indexes, None, common_parser) {
            Ok(v) => match v.len() == 1 {
                true => v[0].to_owned(),
                false => bail!("Found more than one tables with same number of columns"),
            },
            Err(e) => bail!("Failed to find (or read) ISO 3166-1 countries table from wikipedia {}", e),
        };

        let mut scraped = vec![];

        for r in table.rows {
            let mut cs = Self::default();

            for (i, v) in r.into_iter() {
                let val = match v.len() == 1 {
                    true => Some(v[0].to_owned()),
                    false => match i {
                        3 => v.into_iter().find(|s|s.len() == 2),
                        4 => v.into_iter().find(|s|s.len() == 3),
                        7 => v.into_iter().find(|s|s.starts_with(".")),
                        _ => v.into_iter().next(),
                    }
                };
    
                match val {
                    Some(s) => {
                        let v = s.trim_end().to_string();
                        match i {
                            0 => cs.exonym = v,
                            1 => cs.name = v,
                            3 => cs.iso_3166_1_a2 = v,
                            4 => cs.iso_3166_1_a3 = v,
                            7 => cs.tld = v,
                            _ => bail!("Stupid developer with mixed indexes {}", url),
                        }
                    },
                    None => bail!("Well, shit, column index {} from header {} did \
                        not have a value", i, table.headers.get(&i).unwrap())
                }
            }
            scraped.push(cs);
        }

        Ok(scraped)
    }
}

#[derive(Deserialize, Default, Debug)]
pub struct WikipediaScrapedCurrency {
    pub exonym: String,
    pub name: String,
    pub iso_4217: Option<String>,
    pub symbol: Option<String>,
    pub fraction: Option<String>,
    pub fractions_in_unit: Option<i32>,
}

impl WikipediaScrapedCurrency {
    pub fn scrape(url: &str) -> Result<Vec<Self>> {
        println!("Scrape circulating currencies data from wikipedia...");

        // Define takes and discards
        let columns = 6;
        let discard_col_indexes = vec![];
    
        let table = match super::table_scraper(url, columns, discard_col_indexes, None, common_parser) {
            Ok(v) => match v.len() == 1 {
                true => v[0].to_owned(),
                false => bail!("Found more than one tables with same number of columns"),
            },
            Err(e) => bail!("Failed to find (or read) circulating currencies table from wikipedia {}", e),
        };

        let mut scraped = vec![];

        for r in table.rows {
            let mut cs = Self::default();

            for (i, v) in r.into_iter() {
                // The '(none)' case
                let hasval = ! (v.len() >= 3 && v.join("").starts_with("(none)"));

                let val = match v.len() == 1 {
                    true => Some(v[0].to_owned()),
                    false => match i {
                        3 => v.into_iter().find(|s|hasval && s.len() == 3),
                        _ => v.into_iter().next(),
                    }
                };

                match val {
                    Some(s) => {
                        let v = s.trim_end().to_string();
                        let sommed = Some(v.to_owned());

                        match i {
                            0 => cs.exonym = v,
                            1 => cs.name = v,
                            2 => cs.symbol = sommed,
                            3 => cs.iso_4217 = sommed,
                            4 => cs.fraction = sommed,
                            5 => cs.fractions_in_unit = match v.parse::<i32>() {
                                Ok(u) => Some(u),
                                Err(e) => {
                                    eprintln!("Failed to read {} as number: {}", s, e);    
                                    None
                                }
                            },
                            _ => bail!("Stupid developer with mixed indexes {}", url),
                        }
                    },
                    None => match i {
                        2 => cs.symbol = None,
                        3 => cs.iso_4217 = None,
                        4 => cs.fraction = None,
                        5 => cs.fractions_in_unit = None,
                        x => bail!("Well, shit, column index {} from header {} \
                            did not have a value", x, table.headers.get(&x).unwrap())
                    }
                }
            }
            scraped.push(cs);
        }

        Ok(scraped)
    }
}

#[derive(Deserialize, Default, Debug)]
pub struct WikipediaScrapedFlag {
    pub iso_3166_1_a2: String,
    pub flag: String,
}

impl WikipediaScrapedFlag {
    pub fn scrape(url: &str) -> Result<Vec<Self>> {
        println!("Scrape flag emojis data from wikipedia...");

        // Define takes and discards
        let columns = 4;
        let discard_col_indexes = vec![2, 3];
    
        let table = match super::table_scraper(url, columns, discard_col_indexes, None, common_parser) {
            Ok(v) => match v.len() == 1 {
                true => v[0].to_owned(),
                false => bail!("Found more than one tables with same number of columns"),
            },
            Err(e) => bail!("Failed to find (or read) country flags table from wikipedia {}", e),
        };

        let mut scraped = vec![];

        for r in table.rows {
            let mut cs = Self::default();

            for (i, v) in r.into_iter() {
                let val = match i {
                    0 => v.into_iter().next(),
                    1 => v.into_iter().find(|s|s.len() == 2),
                    _ => bail!("Stupid developer with mixed indexes {}", url),
                };

                // let val = match v.len() == 1 {
                //     true => Some(v[0].to_owned()),
                //     false => v.into_iter().next(),
                // };

                match val {
                    Some(s) => {
                        let v = s.trim_end().to_string();

                        match i {
                            0 => cs.flag = v,
                            1 => cs.iso_3166_1_a2 = v,
                            _ => bail!("Stupid developer with mixed indexes {}", url),
                        }
                    },
                    None => bail!("Well, shit, column index {} from header {} did \
                        not have a value", i, table.headers.get(&i).unwrap()),
                }
            }
            scraped.push(cs);
        }

        Ok(scraped)
    }
}

#[derive(Deserialize, Default, Debug)]
pub struct WikipediaScrapedCcTz {
    pub exonym: String,
    pub code: String,
    pub tz: String,
    pub dst: Option<String>
}

impl WikipediaScrapedCcTz {
    pub fn scrape(url: &str) -> Result<Vec<Self>> {
        println!("Scrape calling code and timezone data from wikipedia...");

        // Define takes and discards
        let columns = 4;
        let discard_col_indexes = vec![];
    
        let tables = match super::table_scraper(url, columns, discard_col_indexes, None, common_parser) {
            Ok(v) => v,
            Err(e) => bail!("Failed to find (or read) calling codes and \
                timezones table from wikipedia {}", e),
        };

        let mut scraped = vec![];

        for t in tables {
            // Let's try to determine if this is our table by reading the headers
            let country = t.headers.get(&0).unwrap().to_lowercase().contains("country");
            let code = t.headers.get(&1).unwrap().to_lowercase().contains("code");
            let tz = t.headers.get(&2).unwrap().to_lowercase().contains("zone");

            if ! country || ! code || ! tz {
                println!("Skipping table with headers {:?} as it does not seem \
                    familiar", t.headers);

                continue;
            }

            'row: for r in t.rows {
                let mut cs = Self::default();
    
                for (i, v) in r.into_iter() {
                    let val = match v.len() == 1 {
                        true => Some(v[0].to_owned()),
                        false => match i == 0 {
                            true => v.into_iter().last(),
                            false => v.into_iter().next(),
                        },
                    };
    
                    match val {
                        Some(s) => {
                            let v = s.trim_end().to_string();
    
                            match i {
                                0 => cs.exonym = v,
                                1 => cs.code = v,
                                // Empty timezone seems to be non country calling code
                                2 => cs.tz = match v.is_empty() {
                                    true => continue 'row,
                                    false => v,
                                },
                                3 => cs.dst = match v.is_empty() {
                                    true => None,
                                    false => Some(v),
                                },
                                _ => bail!("Stupid developer with mixed indexes {}", url),
                            }
                        },
                        None => match i {
                            2 => continue 'row,
                            3 => cs.dst = None,
                            x => bail!("Well, shit, column index {} from header {} \
                                did not have a value", x, t.headers.get(&x).unwrap())
                        },
                    }
                }
                scraped.push(cs);
            }
        }

        Ok(scraped)
    }
}


#[derive(Deserialize, Default, Debug)]
pub struct WikipediaScrapedEndonyms {
    pub exonym: String,
    pub capital: String,
    pub endonyms: Vec<String>,
    pub languages: Vec<String>
}

impl WikipediaScrapedEndonyms {
    pub fn scrape(url: &str) -> Result<Vec<Self>> {
        println!("Scrape local country names and spoken languages data from wikipedia...");

        // Define takes and discards
        let columns = 5;
        let discard_col_indexes = vec![3];
        
        // One table per alphabet, collecting countries by their exonym
        let tables = match super::table_scraper(url, columns, discard_col_indexes, None, endonym_lang_parser) {
            Ok(v) => v,
            Err(e) => bail!("Failed to find (or read) endonyms and languages table from wikipedia {}", e),
        };

        let mut scraped = vec![];

        for t in tables {
            // Let's try to determine if this is our table by reading the headers
            let country = t.headers.get(&0).unwrap().to_lowercase().contains("exonym");
            let capital = t.headers.get(&1).unwrap().to_lowercase().contains("exonym");
            let country_endonym = t.headers.get(&2).unwrap().to_lowercase().contains("endonym");
            let lang = t.headers.get(&4).unwrap().to_lowercase().contains("language");

            if ! country || ! capital || ! country_endonym || ! lang {
                println!("Skipping table with headers {:?} as it does not seem \
                    familiar", t.headers);

                continue;
            }

            'row: for r in t.rows {
                let mut cs = Self::default();
    
                for (i, v) in r.into_iter() {
                    // 0 index is the exonym ...or endonym... and 1 is the capital
                    // if those are missing skip the row.
                    if [0, 1].contains(&i) && v.is_empty() {
                        eprintln!("Skip adding endonyms for country {:?} as index {i} \
                            was empty", cs);
                        continue 'row;
                    }

                    // We're expecting lists for all values except country and
                    // capital
                    match i {
                        0 => cs.exonym = v[0].trim_end().to_owned(),
                        1 => cs.capital = v[0].trim_end().to_owned(),
                        2 => cs.endonyms = v.into_iter().map(|s|s.trim_end().to_string()).collect(),
                        4 => cs.languages = v.into_iter().map(|s|s.trim_end().to_string()).collect(),
                        _ => bail!("Stupid developer with mixed indexes {}", url),
                    }
                }
                scraped.push(cs);
            }
        }

        Ok(scraped)
    }
}


fn common_parser(c: ElementRef<'_>, i: usize) -> Option<Vec<&str>> {
    let a = Selector::parse("a").unwrap();

    match c.select(&a).next() {
        Some(l) => match i {
            0 => match l.value().attr("title") {
                Some(s) => Some(vec![s]),
                None => None,
            },
            _ => Some(l.text().collect::<Vec<_>>()),
        },
        None => Some(c.text().collect::<Vec<_>>())
    }
}

fn endonym_lang_parser(c: ElementRef<'_>, i: usize) -> Option<Vec<&str>> {
    let a = Selector::parse("a").unwrap();
    let span = Selector::parse("span").unwrap();
    let mut items = vec![];

    match i {
        0 | 1 => {
            items = match c.select(&a).next() {
                Some(l) => match l.value().attr("title") {
                    Some(s) => vec![s],
                    None => l.text().collect::<Vec<_>>(),
                },
                None => c.text().collect::<Vec<_>>()
            }
        },
        2 => {
            for l in c.select(&span) {
                let children = l.text().collect::<Vec<_>>();
        
                if children.len() == 1 {
                    items.extend(children);
                }
            }
        },
        4 => {
            'lang: for l in c.select(&a) {
                if let Some(i) = l.text().next() {
                    let excl = ["script", "alphabet", "characters"];

                    for x in excl {
                        if i.ends_with(x) { continue 'lang; }
                    }

                    items.push(i);
                }
            }
        },
        _ => (),
    };

    items.sort();
    items.dedup();

    Some(items)
}