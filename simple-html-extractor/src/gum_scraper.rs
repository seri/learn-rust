use crate::types::Article;
use html5gum::{HtmlString, SpanBound, Spanned, Token, Tokenizer};
use std::collections::BTreeMap;

pub struct GumScraper;

impl crate::types::Scraper for GumScraper {
    fn name(&self) -> &'static str {
        "gum_scraper"
    }

    fn scrape(&self, html: &str) -> Result<Vec<Article>, Box<dyn std::error::Error>> {
        scrape(html)
    }
}

fn scrape(html: &str) -> Result<Vec<Article>, Box<dyn std::error::Error>> {
    let mut in_title_wrapper = false;
    let mut in_title_anchor = false;
    let mut extracted_url = String::with_capacity(128);
    let mut result: Vec<Article> = vec![];

    for Ok(token) in Tokenizer::new(html) {
        match token {
            Token::StartTag(tag) => {
                let tag_name = tag.name.as_ref();

                if tag_name == b"span" && includes_class_in_attrs(&tag.attributes, "titleline") {
                    in_title_wrapper = true;
                    extracted_url.clear();
                } else if tag_name == b"a" && in_title_wrapper && extracted_url.is_empty() {
                    in_title_anchor = true;
                    extract_url_in_anchor(&tag.attributes, &mut extracted_url);
                }
            }
            Token::EndTag(tag) => {
                let tag_name = tag.name.as_ref();

                if tag_name == b"a" && in_title_anchor {
                    in_title_anchor = false;
                } else if tag_name == b"span" && in_title_wrapper {
                    in_title_wrapper = false;
                }
            }
            Token::String(text) => {
                if in_title_anchor {
                    result.push(Article {
                        title: String::from_utf8_lossy(&text).to_string(),
                        url: extracted_url.clone(),
                    });
                }
            }
            _ => {}
        }
    }

    Ok(result)
}

fn includes_class_in_attrs<S: SpanBound>(
    attrs: &BTreeMap<HtmlString, Spanned<HtmlString, S>>,
    class_name: &str,
) -> bool {
    if let Some(value) = attrs.get(b"class".as_slice()) {
        return String::from_utf8_lossy(value)
            .split_whitespace()
            .any(|cls| cls == class_name);
    }
    false
}

fn extract_url_in_anchor<S: SpanBound>(
    attrs: &BTreeMap<HtmlString, Spanned<HtmlString, S>>,
    result: &mut String,
) {
    if let Some(value) = attrs.get(b"href".as_slice()) {
        if let Ok(url_str) = std::str::from_utf8(value.as_ref()) {
            result.push_str(url_str);
        }
    }
}
