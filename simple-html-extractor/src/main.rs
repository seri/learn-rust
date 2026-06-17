use std::time::Instant;

mod gum_scraper;
mod rcdom_scraper;
mod types;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get("https://news.ycombinator.com/")?.text()?;

    let scrapers: Vec<&dyn types::Scraper> =
        vec![&rcdom_scraper::RcdomScraper, &gum_scraper::GumScraper];

    for scraper in scrapers {
        println!("== Using {}", scraper.name());

        let timer = Instant::now();
        let articles = scraper.scrape(&response)?;
        let elapsed = timer.elapsed();

        for article in &articles {
            println!("{:?}", article);
        }

        println!(
            "== Scraped {} articles in {} ms",
            articles.len(),
            elapsed.as_millis()
        );
        println!();
    }

    Ok(())
}
