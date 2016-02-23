// TODO: get num cols of terminal dynamically
// TODO: care about the given print options

use super::{Report, ReportKind, RemarkKind};
use code::{FileMap, LineIdx};
use term_painter::ToStyle;
use term_painter::Color::*;
use std::default::Default;

/// Options for printing on the terminal. By `default()` everything is enabled.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PrintOptions {
    /// Use of unicode allowed?
    pub unicode: bool,
    /// Use of colors allowed?
    pub color: bool,
    /// Is line wrapping allowed?
    pub line_wrap: bool,
}

impl Default for PrintOptions {
    fn default() -> Self {
        PrintOptions {
            unicode: true,
            color: true,
            line_wrap: true,
        }
    }
}

/// Pretty prints a report
///
/// **Note**: right now, the `PrintOptions` are ignored.
pub fn print(rep: &Report, src: &FileMap, _: PrintOptions) {
    // print header
    let title = match rep.kind {
        ReportKind::Error => White.bold().bg(Red).paint("ERROR"),
        ReportKind::Warning => White.bold().bg(Yellow).paint("WARNING"),
    };

    let line = if rep.span.is_dummy() {
        "<dummy-span>".into()
    } else {
        let start = src.get_loc(rep.span.lo);
        let end = src.get_loc(rep.span.hi);
        if start.line != end.line {
            format!("{}-{}", start.line, end.line)
        } else {
            start.line.to_string()
        }
    };

    println!("+---- {} in {} : {} ----+",
        title,
        src.filename(),
        Magenta.bold().paint(line)
    );



    for rem in &rep.remarks {
        // print message
        let (title, title_len) = match rem.kind {
            RemarkKind::Error => (Red.paint("error:"), 6),
            RemarkKind::Warning => (Yellow.paint("warning:"), 8),
            RemarkKind::Note => (Green.paint("note:"), 5),
        };

        print!("      =====>  {} ", title);
        // spaces + big arrow + spaces + title + space
        let indent = 6 + 6 + 2 + title_len + 1;
        let block_width = 80 - indent;

        let mut col = 0;
        for word in rem.desc.split_whitespace() {
            let word_len = word.chars().count();
            if col + word_len >= block_width && col != 0 {
                println!("");
                print!("           >  {0:>1$} ", " ", title_len);
                col = 0;
            }
            print!("{} ", White.bold().paint(word));
            col += word_len + 1;
        }
        println!("");

        // print code snippet
        if let Some(span) = rem.span {
            let start = src.get_loc(span.lo);
            let end = src.get_loc(span.hi);

            if span.is_dummy() {
                println!("!!! no snippet due to <dummy-span> !!!");
            } else if start.line == end.line {
                if let Some(line) = src.get_line(start.line) {
                    println!("{:>#4} {} {}",
                        Magenta.bold().paint(start.line),
                        Magenta.bold().paint("|"),
                        line
                    );

                    Yellow.with(|| {
                        println!("       {: <2$}{:-<3$}",
                            " ", "^",
                            start.col.0 as usize,
                            (end.col.0 - start.col.0) as usize
                        );
                    });
                }
            } else {
                for line_idx in start.line.0..end.line.0 + 1 {
                    let line_idx = LineIdx(line_idx);
                    if let Some(line) = src.get_line(line_idx) {
                        println!("{:>#4} {} {}",
                            Magenta.bold().paint(line_idx),
                            Magenta.bold().paint("|"),
                            line
                        );
                    }
                }
            }
            println!("");
        }

    }
    println!("");
}
