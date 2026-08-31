// SPDX-License-Identifier: MPL-2.0

use dosovoi_core::{
    build_view, render_text, AppState, Config, Event, Mode, PresentationOptions, RenderStyle,
    RiskLevel, RiskSignal,
};
use std::{
    env,
    ffi::OsString,
    fmt, fs,
    io::{self, Write},
    path::{Path, PathBuf},
    process::ExitCode,
};

const HELP: &str = "DOSovoi — deterministic, non-agentic terminal mascots

Usage:
  dosovoi [--config PATH] <command>

Commands:
  preview [--pet PET] [--mode MODE] [--ascii|--unicode|--plain] [--no-color]
  mode <serenity|flow|vigilance|detachment|care>
  care <on|off>
  signal <caution|high> --reason TEXT
  signal clear
  status
  enable | disable
  config <path|show|init>
  help

Pets: bunny, puff, lens

Risk comes only from an explicit signal command. Mascot state is not a safety
assessment. NO_COLOR disables ANSI colour. Every command exits immediately.
";

#[derive(Debug)]
struct CliError(String);

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for CliError {}

impl From<io::Error> for CliError {
    fn from(error: io::Error) -> Self {
        Self(error.to_string())
    }
}

fn main() -> ExitCode {
    match run(env::args().skip(1).collect()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("dosovoi: {error}");
            ExitCode::from(2)
        }
    }
}

fn run(mut args: Vec<String>) -> Result<(), CliError> {
    if matches!(
        args.first().map(String::as_str),
        None | Some("help" | "--help" | "-h")
    ) {
        print!("{HELP}");
        return Ok(());
    }
    if matches!(args.first().map(String::as_str), Some("--version" | "-V")) {
        println!("dosovoi {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    let config_path = take_config_path(&mut args)?.unwrap_or_else(default_config_path);
    let config = load_config(&config_path)?;
    let state_path = state_path(&config_path);
    let mut state = load_state(&state_path, &config)?;
    let command = args
        .first()
        .ok_or_else(|| CliError("missing command; try 'dosovoi help'".into()))?
        .clone();
    args.remove(0);

    match command.as_str() {
        "preview" => {
            let (options, mode) = preview_options(&args, &config)?;
            if let Some(mode) = mode {
                if mode == Mode::Care {
                    state.apply(Event::SetCare(true));
                } else {
                    state.apply(Event::SetMode(mode));
                }
            }
            show(&config, &state, options);
        }
        "status" => {
            expect_empty(&args, "status")?;
            show(
                &config,
                &state,
                environment_options(PresentationOptions::from_config(&config)),
            );
        }
        "mode" => {
            let mode: Mode = one_value(&args, "mode <MODE>")?
                .parse()
                .map_err(display_error)?;
            if mode == Mode::Care {
                state.apply(Event::SetCare(true));
            } else {
                state.apply(Event::SetMode(mode));
            }
            save_state(&state_path, &state)?;
            show(
                &config,
                &state,
                environment_options(PresentationOptions::from_config(&config)),
            );
        }
        "care" => {
            let enabled = parse_on_off(one_value(&args, "care <on|off>")?)?;
            state.apply(Event::SetCare(enabled));
            save_state(&state_path, &state)?;
            show(
                &config,
                &state,
                environment_options(PresentationOptions::from_config(&config)),
            );
        }
        "signal" => {
            let signal = parse_signal(&args)?;
            state.apply(Event::SetRisk(signal));
            save_state(&state_path, &state)?;
            show(
                &config,
                &state,
                environment_options(PresentationOptions::from_config(&config)),
            );
        }
        "enable" | "disable" => {
            expect_empty(&args, &command)?;
            state.apply(Event::SetEnabled(command == "enable"));
            save_state(&state_path, &state)?;
            show(
                &config,
                &state,
                environment_options(PresentationOptions::from_config(&config)),
            );
        }
        "config" => config_command(&args, &config_path, &config)?,
        _ => {
            return Err(CliError(format!(
                "unknown command {command:?}; try 'dosovoi help'"
            )))
        }
    }
    Ok(())
}

fn take_config_path(args: &mut Vec<String>) -> Result<Option<PathBuf>, CliError> {
    if args.first().map(String::as_str) != Some("--config") {
        return Ok(env::var_os("DOSOVOI_CONFIG").map(PathBuf::from));
    }
    if args.len() < 2 {
        return Err(CliError("--config requires a path".into()));
    }
    let path = PathBuf::from(args.remove(1));
    args.remove(0);
    Ok(Some(path))
}

fn default_config_path() -> PathBuf {
    if let Some(base) = env::var_os("XDG_CONFIG_HOME") {
        return PathBuf::from(base).join("dosovoi/config");
    }
    if let Some(home) = env::var_os("HOME") {
        return PathBuf::from(home).join(".config/dosovoi/config");
    }
    PathBuf::from(".dosovoi-config")
}

fn state_path(config_path: &Path) -> PathBuf {
    let mut value: OsString = config_path.as_os_str().to_owned();
    value.push(".state");
    PathBuf::from(value)
}

fn load_config(path: &Path) -> Result<Config, CliError> {
    match fs::read_to_string(path) {
        Ok(contents) => Config::parse(&contents).map_err(display_error),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(Config::default()),
        Err(error) => Err(CliError(format!("cannot read {}: {error}", path.display()))),
    }
}

fn load_state(path: &Path, config: &Config) -> Result<AppState, CliError> {
    match fs::read_to_string(path) {
        Ok(contents) => AppState::parse(&contents, config).map_err(display_error),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(AppState::from_config(config)),
        Err(error) => Err(CliError(format!("cannot read {}: {error}", path.display()))),
    }
}

fn save_state(path: &Path, state: &AppState) -> Result<(), CliError> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        fs::create_dir_all(parent)?;
    }
    let mut temporary = path.as_os_str().to_owned();
    temporary.push(format!(".tmp-{}", std::process::id()));
    let temporary = PathBuf::from(temporary);
    fs::write(&temporary, state.serialize())?;
    fs::rename(&temporary, path)?;
    Ok(())
}

fn preview_options(
    args: &[String],
    config: &Config,
) -> Result<(PresentationOptions, Option<Mode>), CliError> {
    let mut options = PresentationOptions::from_config(config);
    let mut mode = None;
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--pet" => {
                index += 1;
                options.mascot = args
                    .get(index)
                    .ok_or_else(|| CliError("--pet requires a value".into()))?
                    .parse()
                    .map_err(display_error)?;
            }
            "--mode" => {
                index += 1;
                mode = Some(
                    args.get(index)
                        .ok_or_else(|| CliError("--mode requires a value".into()))?
                        .parse()
                        .map_err(display_error)?,
                );
            }
            "--ascii" => options.style = RenderStyle::Ascii,
            "--unicode" => options.style = RenderStyle::Unicode,
            "--plain" => options.style = RenderStyle::Plain,
            "--no-color" => options.color = false,
            other => return Err(CliError(format!("unknown preview option {other:?}"))),
        }
        index += 1;
    }
    Ok((environment_options(options), mode))
}

fn environment_options(mut options: PresentationOptions) -> PresentationOptions {
    if env::var_os("NO_COLOR").is_some() {
        options.color = false;
    }
    options
}

fn parse_signal(args: &[String]) -> Result<RiskSignal, CliError> {
    let level: RiskLevel = args
        .first()
        .ok_or_else(|| CliError("signal requires caution, high, or clear".into()))?
        .parse()
        .map_err(display_error)?;
    if level == RiskLevel::None {
        if args.len() != 1 {
            return Err(CliError("signal clear takes no reason".into()));
        }
        return Ok(RiskSignal::clear());
    }
    if args.len() != 3 || args.get(1).map(String::as_str) != Some("--reason") {
        return Err(CliError(
            "signal caution/high requires --reason TEXT".into(),
        ));
    }
    RiskSignal::new(level, args.get(2).cloned()).map_err(display_error)
}

fn show(config: &Config, state: &AppState, options: PresentationOptions) {
    print!("{}", render_text(&build_view(config, state, options)));
}

fn config_command(args: &[String], path: &Path, config: &Config) -> Result<(), CliError> {
    match one_value(args, "config <path|show|init>")? {
        "path" => println!("{}", path.display()),
        "show" => print!("{}", serialize_config(config)),
        "init" => {
            if let Some(parent) = path
                .parent()
                .filter(|parent| !parent.as_os_str().is_empty())
            {
                fs::create_dir_all(parent)?;
            }
            let mut file = fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(path)
                .map_err(|error| {
                    CliError(format!(
                        "cannot create {} without overwriting: {error}",
                        path.display()
                    ))
                })?;
            file.write_all(Config::example().as_bytes())?;
            println!("Created {}", path.display());
        }
        value => return Err(CliError(format!("unknown config command {value:?}"))),
    }
    Ok(())
}

fn serialize_config(config: &Config) -> String {
    format!(
        "enabled = {}\nmascot = {}\nrender = {}\ncolor = {}\nmotion = {}\n\
         default_mode = {}\ncare_plain_text = {}\ncare_disable_color = {}\n",
        config.enabled,
        config.mascot,
        config.render,
        config.color,
        config.motion,
        config.default_mode,
        config.care.plain_text,
        config.care.disable_color
    )
}

fn one_value<'a>(args: &'a [String], usage: &str) -> Result<&'a str, CliError> {
    if args.len() == 1 {
        Ok(&args[0])
    } else {
        Err(CliError(format!("expected {usage}")))
    }
}

fn expect_empty(args: &[String], usage: &str) -> Result<(), CliError> {
    if args.is_empty() {
        Ok(())
    } else {
        Err(CliError(format!("{usage} takes no arguments")))
    }
}

fn parse_on_off(value: &str) -> Result<bool, CliError> {
    match value {
        "on" => Ok(true),
        "off" => Ok(false),
        _ => Err(CliError(format!("expected on or off, got {value:?}"))),
    }
}

fn display_error(error: impl fmt::Display) -> CliError {
    CliError(error.to_string())
}
