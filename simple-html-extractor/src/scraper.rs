pub trait Scraper {
    fn name(&self) -> &'static str;
    fn scrape(&self, html: &str) -> Result<(), Box<dyn std::error::Error>>;
}
