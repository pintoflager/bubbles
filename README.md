# Societies json

Rust bin / lib that combines society information from multiple sources.
See `out.json` for output generated on 3rd of November 2023.

This is just a http client / web scraper that either reads json data from the given url or reads html page and tries to parse table data from there into a more usable format.

Currently uses [wikipedia.org](https://en.wikipedia.org) and [datahub.io](https://www.datahub.io) as a data source. Read more from [lib.rs](./src/lib.rs)

Scraper is opportunistic, meaning it reads every header and data row from each html table it can find if table has the given number of columns (and matching header names if needed).

What's won with that is a scraper that won't die if table is moved around on the page or it's id, classes or attributes are changed.

It's not resistent to columns getting added or removed or in some cases headers getting renamed though.

## How

Install [rust](https://www.rust-lang.org/tools/install) if not yet installed.

clone [this repository](https://github.com/pintoflager/bubbles) and shell into it's directory.
```bash
git clone git@github.com:pintoflager/bubbles.git
cd bubbles
```

To generate `out.json` execute command
```bash
cargo run
```

If data sources are still in place and remained mostly unchanged you should see something like `out.json` on the dir root.

## Note

Works on my linux but I haven't tested this on any other platform. Nothing on the code is intentionally unix only though, so might even compile and run on windows.
