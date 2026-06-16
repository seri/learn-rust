use std::time::Instant;

mod gum_scraper;
mod rcdom_scraper;
mod scraper;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get("https://news.ycombinator.com/")?.text()?;

    let scrapers: Vec<&dyn scraper::Scraper> =
        vec![&rcdom_scraper::RcdomScraper, &gum_scraper::GumScraper];

    for scraper in scrapers {
        println!("== Using {}", scraper.name());
        let timer = Instant::now();
        scraper.scrape(&response)?;
        let elapsed = timer.elapsed();
        println!("== Finished in {} ms", elapsed.as_millis());
        println!();
    }

    Ok(())
}
