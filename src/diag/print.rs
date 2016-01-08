// TODO: get num cols of terminal dynamically

use super::{Report, ReportKind, Remark, RemarkKind};
use code::{FileMap, LineIdx};
use term_painter::{ToStyle, Color, Attr};
use term_painter::Color::*;

/// Options for printing
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PrintOptions {
    /// Use of unicode allowed?
    pub unicode: bool,
    /// Use of colors allowed?
    pub color: bool,
    /// Is line wrapping allowed?
    pub line_wrap: bool,
}


pub fn print(rep: &Report, src: &FileMap, _: PrintOptions) {
    // print header
    let title = match rep.kind {
        ReportKind::Error => White.bold().bg(Red).paint("ERROR"),
        ReportKind::Warning => Attr::Bold.bg(Yellow).paint("WARNING"),
    };

    let start = src.get_loc(rep.span.lo);
    let end = src.get_loc(rep.span.hi);
    let line = if start.line != end.line {
        format!("{}-{}", start.line.0, end.line.0)
    } else {
        start.line.0.to_string()
    };

    println!("+---- {} in {} : {} ----+",
        title,
        src.filename(),
        Blue.paint(line)
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
            print!("{} ", White.paint(word));
            col += word_len + 1;
        }
        println!("");

        // print code snippet
        if let Some(span) = rem.span {
            let start = src.get_loc(span.lo);
            let end = src.get_loc(span.hi);

            if start.line == end.line {
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
