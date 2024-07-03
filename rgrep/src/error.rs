use thiserror::Error;

#[derive(Error, Debug)]
pub enum RgrepError {
    #[error("Glob pattern error")]
    GlobPatternError(#[from] glob::PatternError),
    #[error("Regex pattern error")]
    RegexPatternError(#[from] regex::Error),
    #[error("IO error")]
    IOError(#[from] std::io::Error),
}
