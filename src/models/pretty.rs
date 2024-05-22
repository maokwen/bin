use std::convert::From;
use std::fs;
use std::path::Path;

use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

static SYNTAXES: &[u8] =
    include_bytes!("../../resources/syntaxes/syntaxes.bin");
static THEMES: &[u8] =
    include_bytes!("../../resources/themes/ayu_dark.tmTheme");

pub(crate) enum Error {
    Io(Box<dyn std::error::Error>),
    InvalidSyntax(syntect::Error),
    ThemeLoading(syntect::LoadingError),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(Box::new(e))
    }
}

impl From<syntect::Error> for Error {
    fn from(e: syntect::Error) -> Self {
        Error::InvalidSyntax(e)
    }
}

impl From<syntect::LoadingError> for Error {
    fn from(e: syntect::LoadingError) -> Self {
        Error::ThemeLoading(e)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(e) => write!(f, "IO Error: {}", e),
            Error::InvalidSyntax(e) => write!(f, "Invalid Syntax: {}", e),
            Error::ThemeLoading(e) => write!(f, "Theme Loading Error: {}", e),
        }
    }
}

pub fn get_pretty_body(path: &Path, ext: &str) -> Result<String, Error> {
    let ss: SyntaxSet = syntect::dumps::from_binary(SYNTAXES);

    let mut theme_cursor = std::io::Cursor::new(THEMES);
    let theme = ThemeSet::load_from_reader(&mut theme_cursor)?;

    let content = fs::read_to_string(path)?;
    let syntax = ss
        .find_syntax_by_token(ext)
        .unwrap_or_else(|| ss.find_syntax_plain_text());

    Ok(highlighted_html_for_string(&content, &ss, syntax, &theme)?)
}
