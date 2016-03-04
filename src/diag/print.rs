// TODO: get num cols of terminal dynamically
// TODO: care about the given print options

use super::{Report, ReportKind, RemarkKind, Snippet};
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
    trace!("Printing report: {:#?}", rep);
    trace!("Printing with filemap: {:#?}", src);

    // print header
    let title = match rep.kind {
        ReportKind::Error => White.bold().bg(Red).paint("ERROR"),
        ReportKind::Warning => White.bold().bg(Yellow).paint("WARNING"),
    };

    let (sep, line) = if let Some(span) = rep.span {
        (" : ", if span.is_dummy() {
            "<dummy-span>".into()
        } else {
            let start = src.get_loc(span.lo);
            let end = src.get_loc(span.hi);
            trace!("Span is from {:?} to {:?}", start, end);

            if start.line != end.line {
                format!("{}-{}", start.line, end.line)
            } else {
                start.line.to_string()
            }
        })
    } else {
        ("", "".into())
    };

    println!("+---- {} in {}{}{} ----+",
        title,
        src.filename(),
        sep,
        Magenta.bold().paint(line)
    );



    for rem in &rep.remarks {
        trace!("Handling Remark {:?}", rem);

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
        match rem.snippet {
            Snippet::Orig(_) | Snippet::Replace { .. } => {
                print_snippet(src, &rem.snippet);
                println!("");
            },
            _ => {},
        }
    }
    println!("");
}

fn print_snippet(src: &FileMap, snippet: &Snippet) {
    let span = match *snippet {
        Snippet::Orig(span) => span,
        Snippet::Replace { span, .. } => span,
        _ => unreachable!(),
    };

    let start = src.get_loc(span.lo);
    let end = src.get_loc(span.hi);
    trace!("Span is from {:?} to {:?}", start, end);


    // ----- Dummyspan -----
    if span.is_dummy() {
        println!("   {} {} ! no snippet due to <dummy-span>, this is a bug !",
            Magenta.bold().paint("?"),
            Magenta.bold().paint("|"),
        );
    }

    // ----- Singleline -----
    else if start.line == end.line {
        let line_orig = src
            .get_line(start.line)
            .expect("`Loc` from FileMap should return a valid line");
        trace!("Printing single line span. Orig line: {:?}", line_orig);

        // let mut line = line_orig;
        // // replace string if requested
        // if let Snippet::Replace { replace_span, replace_with, .. } = snippet {

        // }

        // replace tabs
        let line = line_orig.replace("\t", "    ");
        let num_tabs = line_orig[..start.col.0 as usize]
            .chars()
            .filter(|&c| c == '\t')
            .count();

        // adjust cols in case of replaced tabs
        let startcol = start.col.0 as usize + 3*num_tabs;
        let endcol = end.col.0 as usize + 3*num_tabs;

        print!(
            "{:>#4} {} ",
            Magenta.bold().paint(start.line),
            Magenta.bold().paint("|")
        );
        if let &Snippet::Replace { span, ref with } = snippet {
            println!("{}{}{}",
                &line[..startcol],
                Green.paint(with),
                &line[endcol..],
            );

            Green.with(|| {
                println!("      {: <2$}{:^<3$}",
                    " ", "^",
                    startcol + 1,
                    with.len(),
                );
            });
        } else {
            // print line (with marked span section)
            println!("{}{}{}",
                &line[..startcol],
                Yellow.paint(&line[startcol..endcol]),
                &line[endcol..],
            );

            Yellow.with(|| {
                println!("      {: <2$}{:^<3$}",
                    " ", "^",
                    startcol + 1,
                    endcol - startcol,
                );
            });
        }
    }

    // ----- Multiline -----
    else {
        // print first line
        if let Some(line) = src.get_line(start.line) {
            let startcol = start.col.0 as usize;
            println!("{:>#4} {} {}{}",
                Magenta.bold().paint(start.line),
                Magenta.bold().paint("|"),
                &line[..startcol],
                Yellow.paint(&line[startcol..]),
            );
        }


        // print all lines that are completely in the span
        for line_idx in (start.line.0 + 1)..end.line.0 {
            let line_idx = LineIdx(line_idx);
            if let Some(line) = src.get_line(line_idx) {
                println!("{:>#4} {} {}",
                    Magenta.bold().paint(line_idx),
                    Magenta.bold().paint("|"),
                    line
                );
            }
        }

        // print last line
        if let Some(line) = src.get_line(end.line) {
            let endcol = end.col.0 as usize;
            println!("{:>#4} {} {}{}",
                Magenta.bold().paint(end.line),
                Magenta.bold().paint("|"),
                Yellow.paint(&line[..endcol]),
                &line[endcol..],
            );
        }
    }
}
