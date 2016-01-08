use code::Span;

/// Describes some kind of problem or occurrence in the code. Contains one or
/// more remarks with descriptions and separate code spans.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Report {
    pub kind: ReportKind,
    pub span: Span,
    pub remarks: Vec<Remark>,
}

impl Report {
    /// Creates a error report with one message and one span.
    pub fn simple_error<S: Into<String>>(msg: S, span: Span) -> Report {
        Report {
            kind: ReportKind::Error,
            span: span,
            remarks: vec![Remark {
                kind: RemarkKind::Error,
                desc: msg.into(),
                span: Some(span),
            }],
        }
    }

    /// Creates a warning report with one message and one span.
    pub fn simple_warning<S: Into<String>>(msg: S, span: Span) -> Report {
        Report {
            kind: ReportKind::Warning,
            span: span,
            remarks: vec![Remark {
                kind: RemarkKind::Warning,
                desc: msg.into(),
                span: Some(span),
            }],
        }
    }

    /// Adds a note without a span/code snippet to the existing Report
    pub fn with_note<S: Into<String>>(mut self, msg: S) -> Report {
        self.remarks.push(Remark {
            kind: RemarkKind::Note,
            desc: msg.into(),
            span: None,
        });
        self
    }

    /// Adds a note with a span/code snippet to the existing Report
    pub fn with_span_note<S: Into<String>>(mut self, msg: S, span: Span)
        -> Report
    {
        self.remarks.push(Remark {
            kind: RemarkKind::Note,
            desc: msg.into(),
            span: Some(span),
        });
        self
    }
}

/// A report can either be an `Error` or a `Warning`. Still pretty similar to
/// `RemarkType` -- may be merged with it in the future.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ReportKind {
    Error,
    Warning,
}

/// Part of a Report that describes the occurrence with an optional code
/// snippet.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Remark {
    pub kind: RemarkKind,
    pub desc: String,
    pub span: Option<Span>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RemarkKind {
    Error,
    Warning,
    Note,
}
