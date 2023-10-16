use std::fmt::Display;
use std::io;
use std::str::FromStr;

use clap::Parser;
use thiserror::Error;

/// Image conversion utility
#[derive(Parser, Debug)]
#[clap(author = "Louis-Philippe Turmel", version, about, long_about = None)]
pub struct Cli {
    /// The input file to use
    #[clap(long, short)]
    input: String,
    /// The output file to use
    #[clap(long, short)]
    output: String,
}

#[derive(Debug, Error)]
enum Error {
    #[error("Image conversion error: {0}")]
    Image(image::ImageError),
    #[error("File format error: {0}")]
    Extension(String),
    #[error("IO error: {0}")]
    Io(io::Error),
}
impl From<image::ImageError> for Error {
    fn from(err: image::ImageError) -> Self {
        Error::Image(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<String> for Error {
    fn from(err: String) -> Self {
        Error::Extension(err)
    }
}

#[derive(Debug, PartialEq)]
enum Extension {
    Png,
    Jpeg,
    Webp,
}

impl Display for Extension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Extension::Png => write!(f, "png"),
            Extension::Jpeg => write!(f, "jpeg"),
            Extension::Webp => write!(f, "webp"),
        }
    }
}

impl FromStr for Extension {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split('.').collect::<Vec<_>>();
        let s = parts
            .last()
            .cloned()
            .ok_or_else(|| format!("The file {} has no extension, please specify one", s))?;
        match s {
            "png" => Ok(Extension::Png),
            "jpg" => Ok(Extension::Jpeg),
            "jpeg" => Ok(Extension::Jpeg),
            "webp" => Ok(Extension::Webp),
            _ => Err(format!("The extension {} is not supported", s).into()),
        }
    }
}
fn main() {
    if let Err(err) = run() {
        eprintln!("\x1b[31m{}\x1b[0m", err);
        std::process::exit(1);
    }
}
fn run() -> Result<(), Error> {
    let cli = Cli::parse();

    let input_ext = cli.input.parse::<Extension>()?;
    let output_ext = cli.output.parse::<Extension>()?;

    let img = image::open(cli.input)?;

    img.save(cli.output)?;

    println!(
        "Image successfully converted from {} to {}",
        input_ext, output_ext
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::Extension;
    use std::str::FromStr;

    #[test]
    fn test_extension_enum_variants() {
        assert_eq!(format!("{}", Extension::Png), "png");
        assert_eq!(format!("{}", Extension::Jpeg), "jpeg");
        assert_eq!(format!("{}", Extension::Webp), "webp");
    }

    #[test]
    fn test_from_str_valid_extensions() {
        assert_eq!(Extension::from_str("file.png").unwrap(), Extension::Png);
        assert_eq!(Extension::from_str("file.jpg").unwrap(), Extension::Jpeg);
        assert_eq!(Extension::from_str("file.jpeg").unwrap(), Extension::Jpeg);
        assert_eq!(Extension::from_str("file.webp").unwrap(), Extension::Webp);
    }

    #[test]
    fn test_from_str_invalid_extension() {
        assert!(Extension::from_str("file.bmp").is_err());
        assert!(Extension::from_str("file.gif").is_err());
    }

    #[test]
    fn test_from_str_no_extension() {
        assert!(Extension::from_str("file").is_err());
    }

    #[test]
    fn test_from_str_empty_string() {
        assert!(Extension::from_str("").is_err());
    }

    #[test]
    fn test_from_str_multiple_dots() {
        assert_eq!(
            Extension::from_str("file.some.jpg").unwrap(),
            Extension::Jpeg
        );
    }
}
