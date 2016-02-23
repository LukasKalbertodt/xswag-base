extern crate xswag_base as base;

use base::code::{Span, FileMap};
use base::diag;

const TEXT: &'static str = "no-content";

fn main() {
    let file = FileMap::new("src/examples/dummy.txt", TEXT);
    file.find_lines();

    let e = diag::Report::simple_error(
        "Oh noes, this `Report` contains a dummy span :(",
        Span::dummy()
    ).with_span_note(
        "neither does this note :/",
        Span::dummy()
    );

    base::diag::print(&e, &file, diag::PrintOptions::default());
}
