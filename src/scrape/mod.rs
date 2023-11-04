pub mod wikipedia;

use std::collections::BTreeMap;
use anyhow::{Result, bail};
use scraper::*;
use std::ops::Range;

#[derive(Debug, Clone)]
pub struct Table {
    pub headers: BTreeMap<usize, String>,
    pub rows: Vec<BTreeMap<usize, Vec<String>>>
}

impl Table {
    fn new(headers: BTreeMap<usize, String>) -> Self {
        Self { headers, rows: vec![] }
    }
}

pub fn table_scraper(url: &str, columns: usize, discard_col_indexes: Vec<usize>,
fixed_table_index_range: Option<Range<usize>>, parser: impl Fn(ElementRef<'_>, usize) -> Option<Vec<&str>>)
-> Result<Vec<Table>> {
    println!("Scrape table data from {}...", url);

    let document = http_client_reads(url)?;
    let table_selector = Selector::parse("table").unwrap();
    let tr_selector = Selector::parse("tr").unwrap();
    let th_selector = Selector::parse("th").unwrap();
    let td_selector = Selector::parse("td").unwrap();
    let a_selector = Selector::parse("a").unwrap();

    // Select tables from html document
    let table_elements = document.select(&table_selector).collect::<Vec<ElementRef<'_>>>();

    if table_elements.is_empty() {
        bail!("Failed to find tables from {}", url);
    }

    // Collect matches to vector of scraped objects
    let mut scraped = vec![];

    for (g, x) in table_elements.iter().enumerate() {
        if let Some(ref r) = fixed_table_index_range {
            if ! r.contains(&g) {
                println!("Skipping table number {} as it's not within the search range", g);

                continue;
            }
        }
        println!("Iterating html table {}/{} on {}", g + 1, table_elements.len(), url);
        let mut header_map = BTreeMap::new();

        // Rows in table
        let tr_elements = x.select(&tr_selector).collect::<Vec<ElementRef<'_>>>();

        // Loop rows selecting ones containing headers
        for r in tr_elements.iter() {
            // Table headers hold a link we're after
            let th_elements = r.select(&th_selector).collect::<Vec<ElementRef<'_>>>();

            if th_elements.len() != columns {
                continue;
            }

            for (i, h) in th_elements.into_iter().enumerate() {
                // Can be link could be text
                match discard_col_indexes.contains(&i) {
                    true => continue,
                    false => match h.select(&a_selector).next() {
                        Some(l) => match l.value().attr("title") {
                            Some(s) => {
                                header_map.insert(i, s.trim_end().to_string());
                            },
                            None => map_header(&mut header_map, h, i),
                        },
                        None => map_header(&mut header_map, h, i),
                    }
                }
            }
        }

        if header_map.len() != (columns - discard_col_indexes.len()) {
            continue;
        }

        let mut table = Table::new(header_map);

        // Now that we have the headers loop the rows again, this time for content
        for r in tr_elements.into_iter() {
            let data = r.select(&td_selector).collect::<Vec<ElementRef<'_>>>();

            if data.len() != columns { continue; }

            let mut table_row = BTreeMap::new();

            for (i, c) in data.into_iter().enumerate() {
                // Make sure this is not discarded column
                if table.headers.get(&i).is_none() {
                    continue;
                }
    
                let items = match parser(c, i) {
                    Some(v) => v,
                    None => continue,
                };

                table_row.insert(
                    i, 
                    items.into_iter()
                    .map(|s|s.to_owned())
                    .filter(|s|! s.trim_end().is_empty())
                    .collect()
                );
            }

            table.rows.push(table_row);
        }

        scraped.push(table);
    }
    
    return Ok(scraped)
}

fn http_client_reads(url: &str) -> Result<Html> {
    let resp = match reqwest::blocking::get(url) {
        Ok(r) => r,
        Err(e) => bail!("Failed to read http document {}: {}", url, e),
    };

    let html = match resp.text() {
        Ok(s) => s,
        Err(e) => bail!("Failed to open http document from: {}", e),
    };

    // Use scraper to build readable html from response data
    Ok(Html::parse_document(&html))
}

fn map_header(map: &mut BTreeMap<usize, String>, el: ElementRef<'_>, i: usize) {
    let v = el.text().collect::<Vec<_>>();

    if let Some(s) = v.into_iter().next() {
        map.insert(i, s.trim_end().to_string());
    }
}
