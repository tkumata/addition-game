use clap::{arg, Command};

pub struct CliArgs {
    pub timeout: i32,
    pub sound: bool,
}

pub struct UiHandler;

impl UiHandler {
    // ヘルプと引数処理
    pub fn parse_args() -> CliArgs {
        let matches = Command::new("Addition Game")
            .author("Tomokatsu Kumata")
            .about("Addition Game")
            .arg(
                arg!(-t --timeout <TIMEOUT> "Seconds")
                    .default_value("30")
                    .value_parser(clap::value_parser!(i32)),
            )
            .arg(arg!(-s --sound "Enable BGM"))
            .get_matches();

        CliArgs {
            timeout: *matches.get_one::<i32>("timeout").expect("expect number"),
            sound: matches.get_flag("sound"),
        }
    }
}
