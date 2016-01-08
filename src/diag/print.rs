use super::{Report, ReportKind, Remark, RemarkKind};
use code::FileMap;
use term_painter::{ToStyle, Color, Attr};
use term_painter::Color::*;

/// Options for printing
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PrintOptions {
    /// Use of unicode allowed?
    pub unicode: bool,
    /// Use of colors allowed?
    pub color: bool,
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

        // TODO: word wrapping
        for (i, c) in rem.desc.chars().enumerate() {
            if i % block_width == block_width - 1 {
                println!("");
                print!("           >  {0:>1$} ", " ", title_len);
            }
            print!("{}", White.paint(c));
        }
        println!("");

    }
    println!("");
}
