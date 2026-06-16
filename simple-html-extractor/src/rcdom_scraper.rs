use html5ever::Attribute;
use html5ever::ParseOpts;
use html5ever::local_name;
use html5ever::parse_document;
use html5ever::tendril::TendrilSink;
use html5ever::tree_builder::TreeBuilderOpts;
use markup5ever_rcdom::{Handle, NodeData, RcDom};
use std::cell::RefCell;

pub struct RcdomScraper;

impl crate::scraper::Scraper for RcdomScraper {
    fn name(&self) -> &'static str {
        "rcdom_scraper"
    }

    fn scrape(&self, html: &str) -> Result<(), Box<dyn std::error::Error>> {
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

        find_articles(&dom.document, 0);

        Ok(())
    }
}

fn find_articles(node: &Handle, depth: usize) {
    if let NodeData::Element { name, attrs, .. } = &node.data {
        if name.local == local_name!("span") && includes_class_in_attrs(attrs, "titleline") {
            extract_article_in_span(node)
        }
    }

    for child in node.children.borrow().iter() {
        find_articles(child, depth + 1);
    }
}

// We are inside a <span> that has an <a> and this <a> has both the title and the url
fn extract_article_in_span(node: &Handle) {
    for child in node.children.borrow().iter() {
        if let NodeData::Element { name, attrs, .. } = &child.data {
            if name.local == local_name!("a") {
                extract_title_in_anchor(child);
                extract_url_in_anchor(attrs);
                return;
            }
        }
    }
}

fn extract_title_in_anchor(node: &Handle) {
    for child in node.children.borrow().iter() {
        if let NodeData::Text { contents } = &child.data {
            println!("{}", contents.borrow())
        }
        return;
    }
}

fn includes_class_in_attrs(attrs: &RefCell<Vec<Attribute>>, class_name: &str) -> bool {
    attrs.borrow().iter().any(|attr| {
        attr.name.local == local_name!("class") && includes_class(&attr.value, class_name)
    })
}

fn includes_class(haystack: &str, needle: &str) -> bool {
    haystack.split_whitespace().any(|cls| cls == needle)
}

fn extract_url_in_anchor(attrs: &RefCell<Vec<Attribute>>) {
    for attr in attrs.borrow().iter() {
        if attr.name.local == local_name!("href") {
            println!("  {}", attr.value)
        }
    }
}
