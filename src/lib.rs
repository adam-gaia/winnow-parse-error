use std::{error::Error, fmt::Display};
use winnow::error::ContextError;

#[derive(Debug, PartialEq, Eq)]
pub struct ParseError {
    message: String,
    span: std::ops::Range<usize>,
    input: String,
}

impl ParseError {
    // Avoiding `From` so `winnow` types don't become part of our public API
    pub fn from_parse(error: winnow::error::ParseError<&str, ContextError>) -> Self {
        let message = error.inner().to_string();
        let input = (*error.input()).to_owned();
        let span = error.char_span();
        Self {
            message,
            span,
            input,
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = annotate_snippets::Level::Error
            .title(&self.message)
            .snippet(
                annotate_snippets::Snippet::source(&self.input)
                    .fold(true)
                    .annotation(annotate_snippets::Level::Error.span(self.span.clone())),
            );
        let renderer = annotate_snippets::Renderer::plain();
        let rendered = renderer.render(message);
        rendered.fmt(f)
    }
}

impl Error for ParseError {}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    // TODO
}
