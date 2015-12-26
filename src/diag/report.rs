use code::Span;

/// Describes some kind of problem or occurrence in the code. Contains one or
/// more remarks with descriptions and separate code spans.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Report {
    kind: ReportKind,
    span: Span,
    remarks: Vec<Remark>,
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
    kind: RemarkKind,
    desc: String,
    code_snippet: Option<Span>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RemarkKind {
    Error,
    Warning,
    Note,
}
