extern crate reqwest;
extern crate select;

use select::{
    document::Document, predicate::{Class, Name},
};

fn main() {
    let document = Document::from("");
    for node in document.find(Class("foo")) {
        let mut img_url = None;
        for img_node in node.find(Name("bar")) {
            if let Some(img_link) = img_node.attr("src") {
                img_url = img_link.as_ref();
                break;
            }
        }
    }
}
