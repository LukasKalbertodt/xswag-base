extern crate xswag_base as base;
extern crate env_logger;

use base::code::{SrcOffset, BytePos, Span, FileMap};
use base::diag::{Report, Remark, Snippet};

const TEXT: &'static str = "\
You're waiting for a train.
A train that'll take you far cheese
	You know where you hope this triain will take you.
But you can't know for sure. Yet it doesn't matter.
How can it not matter to you where that train will take you? Because...";

fn main() {
    env_logger::init().unwrap();

    let file = FileMap::new("src/examples/train.txt", TEXT);
    file.find_lines();

    let first = TEXT.find("train").unwrap() as SrcOffset;
    let second = TEXT.find("triain").unwrap() as SrcOffset;

    let block_lo = TEXT.find("Yet it").unwrap() as SrcOffset;
    let block_hi = TEXT.find("? Because").unwrap() as SrcOffset + 1;

    let cheese = TEXT.find("cheese").unwrap() as SrcOffset;


    let e = Report::simple_error(
        "unknown symbol `trian`. Did you mean `train`?",
        Span::new(BytePos(second), BytePos(second + 6)),
    ).with_span_note(
        "symbol `train` was previously defined here",
        Span::new(BytePos(first), BytePos(first + 5)),
    );

    let cheese_span = Span::new(BytePos(cheese), BytePos(cheese + 6));
    let e2 = Report::simple_error( "incorrect quote from movie", cheese_span)
        .with_remark(Remark::note(
            "consider replacing it as shown below",
            Snippet::Replace { span: cheese_span, with: "away.".into() },
        ));

    let w = Report::simple_warning(
        "that's a good question. I would be terrified the whole time not \
        knowing where that train would take me. I mean: it could be a bad \
        place! I really hope I triggered a line break by now...",
        Span::new(BytePos(block_lo), BytePos(block_hi)),
    ).with_note("maybe because you are together?");

    let opts = base::diag::PrintOptions::default();
    base::diag::print(&e, &file, opts);
    base::diag::print(&e2, &file, opts);
    base::diag::print(&w, &file, opts);
}
