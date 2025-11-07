use std::{
    process::ExitCode,
    time::Instant,
};

use colored::{
    Color,
    Colorize,
};
use libgoboscript::frontend::frontend;

fn main() -> ExitCode {
    pretty_env_logger::init();
    std::panic::set_hook(Box::new(|info| {
        eprintln!(
            "{info}\n{}\nopen an issue at {}",
            "goboscript is cooked 💀".red().bold(),
            "https://github.com/aspizu/goboscript/issues".cyan()
        );
    }));
    let begin = Instant::now();
    let result = frontend();
    let color = if matches!(result, ExitCode::SUCCESS) {
        Color::Green
    } else {
        Color::Red
    };
    eprintln!(
        "{} in {:?}",
        "Finished".color(color).bold(),
        begin.elapsed()
    );
    result
}

/// Test
#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use colored::Colorize;

    use libgoboscript::frontend::build::build;

    #[test]
    fn test_build() {
        let input = PathBuf::from("tests/newTest");
        let output = PathBuf::from("tests/newTest/newTest1.sb3");
        let result = build(Some(input), None, Some(String::from("TurboWrap")));
        match result {
            Ok(artifact) => {
                artifact.eprint();
                eprintln!();
            }
            Err(err) => {
                eprintln!("{}: {:?}", "error".red().bold(), err);
            }
        }
    }
}
