use std::process::ExitCode;

const HELP: &str = "Melly native runtime (early scaffold)\n\n\
Usage:\n  melly [OPTIONS]\n\n\
Options:\n  -h, --help       Print help\n  -V, --version    Print version\n\n\
The runtime, Servo integration, and host backends are not implemented yet.\n\
See docs/roadmap.md for the staged implementation plan.";

fn main() -> ExitCode {
    match run(std::env::args().skip(1)) {
        Ok(message) => {
            println!("{message}");
            ExitCode::SUCCESS
        }
        Err(message) => {
            eprintln!("error: {message}");
            eprintln!("Try 'melly --help' for more information.");
            ExitCode::from(2)
        }
    }
}

fn run(mut args: impl Iterator<Item = String>) -> Result<String, String> {
    let Some(argument) = args.next() else {
        return Ok(HELP.to_owned());
    };

    if args.next().is_some() {
        return Err("this scaffold accepts only one option".to_owned());
    }

    match argument.as_str() {
        "-h" | "--help" => Ok(HELP.to_owned()),
        "-V" | "--version" => Ok(format!("melly {}", env!("CARGO_PKG_VERSION"))),
        _ => Err(format!("unrecognized option '{argument}'")),
    }
}

#[cfg(test)]
mod tests {
    use super::{HELP, run};

    #[test]
    fn no_arguments_shows_help() {
        assert_eq!(run(std::iter::empty()).unwrap(), HELP);
    }

    #[test]
    fn version_uses_package_version() {
        assert_eq!(
            run(["--version".to_owned()].into_iter()).unwrap(),
            format!("melly {}", env!("CARGO_PKG_VERSION"))
        );
    }

    #[test]
    fn unknown_option_is_an_error() {
        assert!(run(["--unknown".to_owned()].into_iter()).is_err());
    }
}
