use clap::{Parser, ValueEnum};
use std::sync::Arc;
use std::sync::mpsc;
use std::thread;
use std::time::Instant;

mod gum_scraper;
mod rcdom_scraper;
mod types;

const DEFAULT_PAGE_COUNT: i32 = 5;
const PAGE_SIZE: usize = 30;

#[derive(Debug, Clone, Copy, ValueEnum)]
enum ScraperChoice {
    Gum,
    Rcdom,
}

#[derive(Debug, Parser)]
#[command(
    author,
    version,
    about = "A blazing fast multi-threaded Hacker News scraper"
)]
struct Args {
    /// The HTML parsing engine to use
    #[arg(short, long, value_enum, default_value_t=ScraperChoice::Gum)]
    scraper: ScraperChoice,
    /// The number of Hacker News pages to fetch and parse
    #[arg(short, long, default_value_t=DEFAULT_PAGE_COUNT)]
    pages: i32,
}

fn scrape_page(
    scraper: Arc<dyn types::Scraper>,
    sender: mpsc::Sender<Vec<types::Article>>,
    page: i32,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let url = format!("https://news.ycombinator.com/?p={}", page);
    let html = reqwest::blocking::get(&url)?.text()?;
    let _ = sender.send(scraper.scrape(&html)?);
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let pages = args.pages;
    let scraper: Arc<dyn types::Scraper> = match args.scraper {
        ScraperChoice::Gum => Arc::new(gum_scraper::GumScraper),
        ScraperChoice::Rcdom => Arc::new(rcdom_scraper::RcdomScraper),
    };
    println!("== Using {}", scraper.name());

    let timer = Instant::now();
    let (sender, receiver) = mpsc::channel::<Vec<types::Article>>();

    for page in 1..=pages {
        let thread_scraper = scraper.clone();
        let thread_sender = sender.clone();
        thread::spawn(move || scrape_page(thread_scraper, thread_sender, page));
    }

    drop(sender);

    let mut all_articles = Vec::with_capacity(PAGE_SIZE * (pages as usize));
    for articles in receiver {
        all_articles.extend(articles);
    }

    let elapsed = timer.elapsed().as_millis();
    println!("== Scraped {} articles in {}ms", all_articles.len(), elapsed);

    Ok(())
}
