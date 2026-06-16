use html5gum::{HtmlString, SpanBound, Spanned, Token, Tokenizer};
use std::collections::BTreeMap;

pub struct GumScraper;

impl crate::scraper::Scraper for GumScraper {
    fn name(&self) -> &'static str {
        "gum_scraper"
    }

    fn scrape(&self, html: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut in_title_wrapper = false;
        let mut in_title_anchor = false;
        let mut extracted_url = "".to_string();

        for Ok(token) in Tokenizer::new(html) {
            match token {
                Token::StartTag(tag) => {
                    let tag_name = String::from_utf8_lossy(&tag.name);

                    if tag_name == "span" && includes_class_in_attrs(&tag.attributes, "titleline") {
                        in_title_wrapper = true;
                        extracted_url = "".to_string();
                    } else if tag_name == "a" && in_title_wrapper && extracted_url.is_empty() {
                        in_title_anchor = true;
                        extracted_url = extract_url_in_anchor(&tag.attributes);
                    }
                }
                Token::EndTag(tag) => {
                    let tag_name = String::from_utf8_lossy(&tag.name);

                    if tag_name == "a" && in_title_anchor {
                        in_title_anchor = false;
                    } else if tag_name == "span" && in_title_wrapper {
                        in_title_wrapper = false;
                    }
                }
                Token::String(text) => {
                    if in_title_anchor {
                        println!("{}", String::from_utf8_lossy(&text));
                        println!("  {}", extracted_url);
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }
}

fn includes_class_in_attrs<S: SpanBound>(
    attrs: &BTreeMap<HtmlString, Spanned<HtmlString, S>>,
    class_name: &str,
) -> bool {
    attrs.iter().any(|(key, value)| {
        String::from_utf8_lossy(key) == "class"
            && String::from_utf8_lossy(value)
                .split_whitespace()
                .any(|cls| cls == class_name)
    })
}

fn extract_url_in_anchor<S: SpanBound>(
    attrs: &BTreeMap<HtmlString, Spanned<HtmlString, S>>,
) -> String {
    for (key, value) in attrs {
        if String::from_utf8_lossy(key) == "href" {
            return String::from_utf8_lossy(value).to_string();
        }
    }
    "".to_string()
}
