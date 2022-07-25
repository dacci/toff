use clap::Parser;
use shell_escape::escape;
use std::borrow::Cow;

fn main() {
    let args = Args::parse();
    let command: Vec<Cow<str>> = match &args {
        Args::Curl(args) => args.into(),
    };

    print!("{}", command.join(" "));
}

#[derive(Debug, Parser)]
#[clap(version, about)]
enum Args {
    /// From "Copy as cURL".
    Curl(CurlArgs),
}

#[derive(Debug, clap::Args)]
struct CurlArgs {
    url: String,

    /// Extra header to include in the request when sending HTTP to a server.
    #[clap(short = 'H', long)]
    header: Vec<String>,

    /// Ignored.
    #[clap(long)]
    compressed: bool,
}

impl<'a> From<&'a CurlArgs> for Vec<Cow<'a, str>> {
    fn from(args: &'a CurlArgs) -> Self {
        let mut vec: Vec<Cow<str>> = vec![];

        if !args.header.is_empty() {
            let headers = args.header.iter().fold(String::new(), |mut acc, x| {
                acc.push_str(x);
                acc.push_str("\\r\\n");
                acc
            });
            let headers = format!("${}", escape(headers.into()));

            vec.push("-headers".into());
            vec.push(headers.into())
        }

        vec.push("-i".into());
        vec.push(escape(args.url.as_str().into()));

        vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_curl_args() {
        let args = CurlArgs {
            url: "https://example.org/".to_string(),
            header: vec!["Connection: closed".to_string()],
            compressed: false,
        };
        let cmd: Vec<Cow<str>> = (&args).into();
        assert_eq!(
            cmd,
            [
                "-headers",
                "$'Connection: closed\\r\\n'",
                "-i",
                "'https://example.org/'"
            ]
        );
    }
}
