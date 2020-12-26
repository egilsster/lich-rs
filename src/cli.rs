use clap::Clap;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");

/// Make sure your third party dependencies have been approved.
#[derive(Clap)]
#[clap(name = NAME, version = VERSION, author = "Egill Sveinbjörnsson <egillsveinbjorns@gmail.com>")]
pub struct CliArgs {
    /// Path to the package dependency manifest file
    #[clap(value_name = "DEPENDENCY FILE", required = true)]
    pub deps: String,
    /// The Github repo to read approved licenses from
    #[clap(short, long = "repo", value_name = "REPO", required = true)]
    pub repo: String,
    /// The Github repository owner
    #[clap(short, long = "owner", value_name = "OWNER", required = true)]
    pub owner: String,
    /// Which branch of the repository to use
    #[clap(short, long = "branch", value_name = "BRANCH", default_value = "main")]
    pub branch: String,
    /// The Github personal access token (required for private repositories)
    #[clap(short, long = "token", value_name = "TOKEN")]
    pub token: Option<String>,
}
