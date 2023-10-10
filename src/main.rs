use clap::{Parser, Subcommand};
use tomllib::types::{ParseResult, Value};


#[derive(Parser)]
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
struct Cli {
    #[command(subcommand)]
    dummy: Dummy,
}

#[derive(Subcommand)]
enum Dummy {
    Incver {
        #[command(subcommand)]
        part: Part
    },
}

#[derive(Subcommand)]
enum Part {
    #[command(subcommand)]
    Major(Action),
    #[command(subcommand)]
    Minor(Action),
    #[command(subcommand)]
    Patch(Action),
    #[command(subcommand)]
    Build(StringAction),
    #[command(subcommand)]
    Pre(StringAction),
    #[command(subcommand)]
    Full(StringAction)
}


#[derive(Subcommand)]
enum Action {
    /// print value
    Get,
    /// increment value
    Inc,
    /// decement value
    Dec,
    /// set custom value
    Set {
        /// u64 number
        value: u64
    },
}

#[derive(Subcommand)]
enum StringAction {
    /// print value
    Get,
    /// reset value (pre to empty, build to empty, full to 0.1.0)
    Reset,
    /// set custom value
    Set {
        value: String
    }
}

fn main() {
    let Cli { dummy: Dummy::Incver { part } } = Cli::parse();

    let cargo_toml = std::fs::read_to_string("Cargo.toml").expect("Cargo.toml in current directory");
    let (mut parser, result) = tomllib::TOMLParser::new().parse(&cargo_toml);
    match result {
        ParseResult::Full => {},
        other => panic!("parser returns not full result: {:?}", other)
    }

    let mut version = match parser.get_value("package.version").expect("package.version has to be in Cargo.toml") {
        tomllib::types::Value::String(str, _) => {
            semver::Version::parse(&str).unwrap()
        },
        other => panic!("version must be a string, but `{:?}` found", other)
    };

    match part {
        Part::Major(action) => {
            match action {
                Action::Get => {println!("{}", version.major); return;},
                Action::Inc => version.major += 1,
                Action::Dec => version.major = version.major.checked_sub(1).expect("version value can't be less than zero"),
                Action::Set { value } => version.major = value,
            }
        },
        Part::Minor(action) => {
            match action {
                Action::Get => {println!("{}", version.minor); return;},
                Action::Inc => version.minor += 1,
                Action::Dec => version.minor = version.minor.checked_sub(1).expect("version value can't be less than zero"),
                Action::Set { value } => version.minor = value,
            }
        },
        Part::Patch(action) => {
            match action {
                Action::Get => {println!("{}", version.patch); return;},
                Action::Inc => version.patch += 1,
                Action::Dec => version.patch = version.patch.checked_sub(1).expect("version value can't be less than zero"),
                Action::Set { value } => version.patch = value,
            }
        },
        Part::Build(action) => {
            match action {
                StringAction::Get => {println!("{}", version.build); return;},
                StringAction::Set { value } => version.build = semver::BuildMetadata::new(&value).unwrap(),
                StringAction::Reset => version.build = semver::BuildMetadata::EMPTY,
            }
        },
        Part::Pre(action) => {
            match action {
                StringAction::Get => {println!("{}", version.pre); return;},
                StringAction::Set { value } => version.pre = semver::Prerelease::new(&value).unwrap(),
                StringAction::Reset => version.pre = semver::Prerelease::EMPTY,
            }
        }
        Part::Full(action) => {
            match action {
                StringAction::Get => {println!("{}", version); return;},
                StringAction::Reset => version = semver::Version::new(0, 1, 0),
                StringAction::Set { value } => version = semver::Version::parse(&value).unwrap(),
            }
        },
    }

    parser.set_value("package.version", Value::basic_string(version.to_string()).expect("version.to_string() has to be valid string"));
    std::fs::write("Cargo.toml", parser.to_string()).expect("write to Cargo.toml has to be successful");
}
