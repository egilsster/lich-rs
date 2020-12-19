use clap::{App, Arg, ArgMatches};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const NAME: &'static str = env!("CARGO_PKG_NAME");

pub fn parse_parameters() -> ArgMatches {
    let matches = App::new(NAME)
        .version(VERSION)
        .author("Egill Sveinbjörnsson <egillsveinbjorns@gmail.com>")
        .about("Make sure your third party dependencies have been approved")
        .arg(
            Arg::new("deps")
                .value_name("DEPENDENCY FILE")
                .about("path to the package dependency manifest file")
                .required(true),
        )
        .arg(
            Arg::new("repo")
                .long("repo")
                .value_name("REPO")
                .about("the Github repo to read approved licenses from")
                .required(true),
        )
        .arg(
            Arg::new("owner")
                .long("owner")
                .value_name("OWNER")
                .about("the Github repository owner")
                .required(true),
        )
        .arg(
            Arg::new("branch")
                .long("branch")
                .value_name("BRANCH")
                .about("which branch of the repository to use")
                .default_value("main")
                .required(false),
        )
        .arg(
            Arg::new("token")
                .long("token")
                .value_name("TOKEN")
                .about("the Github personal access token (required for private repositories)")
                .required(false),
        )
        .get_matches();

    return matches;
}
