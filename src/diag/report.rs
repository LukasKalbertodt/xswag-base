use code::Span;

/// Describes some kind of problem or occurrence in the code. Contains one or
/// more remarks with descriptions and separate code spans.
///
/// This type doesn't provide a `Display` impl, since all spans reference an
/// external filemap which needs to be provided. Use `print` methods of the
/// `diag` module instead.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Report {
    /// Kind of the report (usually the same as the first remark kind)
    pub kind: ReportKind,
    /// Span of the main code snippet
    pub span: Span,
    /// List of remarks describing the report
    pub remarks: Vec<Remark>,
}

impl Report {
    /// Creates a error report with one message and one span
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

    /// Creates a warning report with one message and one span
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
    /// Something went very wrong and will stop further processing
    Error,
    /// Something important should be fixed, but doesn't stop processing
    Warning,
}

/// Part of a Report that describes the occurrence with an optional code
/// snippet.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Remark {
    pub kind: RemarkKind,
    /// Remark description
    pub desc: String,
    pub span: Option<Span>,
}

/// Kinds of remarks
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RemarkKind {
    /// Something went very wrong and will stop further processing
    Error,
    /// Something important should be fixed, but doesn't stop processing
    Warning,
    /// Additional information about an error or a warning
    Note,
}
