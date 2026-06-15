use html5ever::Attribute;
use html5ever::ParseOpts;
use html5ever::local_name;
use html5ever::parse_document;
use html5ever::tendril::TendrilSink;
use html5ever::tree_builder::TreeBuilderOpts;
use markup5ever_rcdom::{Handle, NodeData, RcDom};
use std::cell::RefCell;

// #[derive(Debug)]
// struct Article {
//     title: String,
//     url: String,
// }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get("https://news.ycombinator.com/")?.text()?;

    let options = ParseOpts {
        tree_builder: TreeBuilderOpts {
            drop_doctype: true,
            ..Default::default()
        },
        ..Default::default()
    };

    let dom = parse_document(RcDom::default(), options)
        .from_utf8()
        .read_from(&mut response.as_bytes())?;

    find_articles(&dom.document, 0);

    Ok(())
}

fn find_articles(node: &Handle, depth: usize) {
    match &node.data {
        NodeData::Element { name, attrs, .. } => {
            if name.local == local_name!("span") && includes_class_in_attrs(attrs, "titleline") {
                extract_article_in_span(node)
            }
        }
        _ => {}
    }

    for child in node.children.borrow().iter() {
        find_articles(child, depth + 1);
    }
}

// We are inside a <span> that has an <a> and this <a> has both the title and the url
fn extract_article_in_span(node: &Handle) {
    for child in node.children.borrow().iter() {
        if let NodeData::Element { name, .. } = &child.data {
            if name.local == local_name!("a") {
                extract_title_in_anchor(child);
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
