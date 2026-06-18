#[derive(Debug)]
pub struct Article {
    pub title: String,
    pub url: String,
}

pub trait Scraper: Send + Sync {
    fn name(&self) -> &'static str;
    fn scrape(&self, html: &str) -> Result<Vec<Article>, Box<dyn std::error::Error + Send + Sync>>;
}
