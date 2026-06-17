use clap::{Parser, ValueEnum};
use std::time::Instant;

mod gum_scraper;
mod rcdom_scraper;
mod types;

#[derive(Debug, Clone, Copy, ValueEnum)]
enum ScraperChoice {
    Gum,
    Rcdom,
}

#[derive(Debug, Parser)]
#[command(author, version, about = "A blazing fast multi-threader web scraper")]
struct Args {
    #[arg(short, long, value_enum, default_value_t = ScraperChoice::Gum)]
    scraper: ScraperChoice,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let scraper: Box<dyn types::Scraper> = match args.scraper {
        ScraperChoice::Gum => Box::new(gum_scraper::GumScraper),
        ScraperChoice::Rcdom => Box::new(rcdom_scraper::RcdomScraper),
    };
    println!("== Using {}", scraper.name());

    let response = reqwest::blocking::get("https://news.ycombinator.com/")?.text()?;
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

    Ok(())
}
