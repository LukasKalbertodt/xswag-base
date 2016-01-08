extern crate xswag_base as base;

use base::code::{SrcOffset, BytePos, Span, FileMap};
use base::diag::Report;

const TEXT: &'static str = "\
You're waiting for a train.
A train that'll take you far away.
You know where you hope this triain will take you.
But you can't know for sure. Yet it doesn't matter.
How can it not matter to you where that train will take you?";

fn main() {
    let file = FileMap::new("src/examples/train.txt", TEXT);
    file.find_lines();

    let first = TEXT.find("train").unwrap() as SrcOffset;
    let second = TEXT.find("triain").unwrap() as SrcOffset;

    let block_lo = TEXT.find("But you can't").unwrap() as SrcOffset;
    let block_hi = TEXT.len() as SrcOffset;


    let e = Report::simple_error(
        "unknown symbol `trian`. Did you mean `train`?",
        Span { lo: BytePos(second), hi: BytePos(second + 6) }
    ).with_span_note(
        "symbol `train` was previously defined here",
        Span { lo: BytePos(first), hi: BytePos(first + 5) }
    );

    let w = Report::simple_warning(
        "that's a good question. I would be terrified the whole time not \
        knowing where that train would take me. I mean: it could be a bad \
        place! I really hope I triggered a line break by now...",
        Span { lo: BytePos(block_lo), hi: BytePos(block_hi) }
    ).with_note("maybe because you are together?");

    let opts = base::diag::PrintOptions {
        unicode: true,
        color: true,
        line_wrap: true
    };
    base::diag::print(&e, &file, opts);
    base::diag::print(&w, &file, opts);
}
