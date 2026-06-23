use std::io::{self, IsTerminal};

use rvtest::core::ColorChoice;

use super::args::Cli;

pub fn resolve_profile(args: &mut Cli) {
    let name = args.profile.clone().or_else(|| {
        std::env::var("RVTEST_PROFILE")
            .ok()
            .filter(|s| !s.is_empty())
    });

    let Some(name) = name else { return };

    match name.as_str() {
        "ci" => {
            if args.format == "pretty" {
                args.format = "junit".into();
            }
            args.fail_fast = true;
            args.verbose = false;
            args.show_output = false;
        }
        "dev" => {
            if args.format == "pretty" {
                args.format = "pretty".into();
            }
            args.verbose = true;
        }
        other => {
            eprintln!("warning: unknown profile '{other}', ignoring");
        }
    }
}

pub fn resolve_color(cli_color: Option<&str>) -> ColorChoice {
    if let Some(c) = cli_color {
        return c.parse().unwrap_or(ColorChoice::Auto);
    }
    if let Ok(val) = std::env::var("CARGO_TERM_COLOR") {
        match val.as_str() {
            "always" => return ColorChoice::Always,
            "never" => return ColorChoice::Never,
            _ => {}
        }
    }
    ColorChoice::Auto
}

pub fn use_color(color: ColorChoice) -> bool {
    match color {
        ColorChoice::Always => true,
        ColorChoice::Never => false,
        ColorChoice::Auto => io::stdout().is_terminal(),
    }
}

pub fn coloured_str(s: &str, code: &str, enabled: bool) -> String {
    if enabled {
        format!("\x1b[{code}m{s}\x1b[0m")
    } else {
        s.to_owned()
    }
}

pub fn dim(s: &str) -> String {
    format!("\x1b[2m{s}\x1b[0m")
}
