extern crate reqwest;
extern crate select;

use select::{document::Document, predicate::Class};

fn main() {
    let document = Document::from("");
    for node in document.find(Class("foo")) {
        let mut img_url = None;
        if let Some(img_link) = node.attr("src") {
            img_url = img_link.as_ref();
        }
    }
}
