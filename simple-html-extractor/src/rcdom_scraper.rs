use crate::types::Article;
use html5ever::Attribute;
use html5ever::ParseOpts;
use html5ever::local_name;
use html5ever::parse_document;
use html5ever::tendril::TendrilSink;
use html5ever::tree_builder::TreeBuilderOpts;
use markup5ever_rcdom::{Handle, NodeData, RcDom};
use std::cell::RefCell;

pub struct RcdomScraper;

impl crate::types::Scraper for RcdomScraper {
    fn name(&self) -> &'static str {
        "rcdom_scraper"
    }

    fn scrape(&self, html: &str) -> Result<Vec<Article>, Box<dyn std::error::Error>> {
        scrape(html)
    }
}

fn scrape(html: &str) -> Result<Vec<Article>, Box<dyn std::error::Error>> {
    let mut result: Vec<Article> = vec![];

    let options = ParseOpts {
        tree_builder: TreeBuilderOpts {
            drop_doctype: true,
            ..Default::default()
        },
        ..Default::default()
    };

    let dom = parse_document(RcDom::default(), options)
        .from_utf8()
        .read_from(&mut html.as_bytes())?;

    find_articles(&dom.document, &mut result);

    Ok(result)
}

fn find_articles(node: &Handle, result: &mut Vec<Article>) {
    if let NodeData::Element { name, attrs, .. } = &node.data {
        if name.local == local_name!("span") && includes_class_in_attrs(attrs, "titleline") {
            if let Some(article) = extract_article_in_span(node) {
                result.push(article)
            }
        }
    }

    for child in node.children.borrow().iter() {
        find_articles(child, result);
    }
}

// We are inside a <span> that has an <a> and this <a> has both the title and the url
fn extract_article_in_span(node: &Handle) -> Option<Article> {
    for child in node.children.borrow().iter() {
        if let NodeData::Element { name, attrs, .. } = &child.data {
            if name.local == local_name!("a") {
                return Some(Article {
                    title: extract_title_in_anchor(child),
                    url: extract_url_in_anchor(attrs),
                });
            }
        }
    }
    None
}

fn extract_title_in_anchor(node: &Handle) -> String {
    for child in node.children.borrow().iter() {
        if let NodeData::Text { contents } = &child.data {
            return contents.borrow().to_string();
        }
    }
    "".to_string()
}

fn includes_class_in_attrs(attrs: &RefCell<Vec<Attribute>>, class_name: &str) -> bool {
    attrs.borrow().iter().any(|attr| {
        attr.name.local == local_name!("class") && includes_class(&attr.value, class_name)
    })
}

fn includes_class(haystack: &str, needle: &str) -> bool {
    haystack.split_whitespace().any(|cls| cls == needle)
}

fn extract_url_in_anchor(attrs: &RefCell<Vec<Attribute>>) -> String {
    for attr in attrs.borrow().iter() {
        if attr.name.local == local_name!("href") {
            return attr.value.to_string();
        }
    }
    "".to_string()
}
