mod cli;
mod compare;
mod github;
mod parser;
mod types;

use clap::Clap;
use cli::CliArgs;
use eyre::Result;
use github::{get_approved_dependencies, get_project_api_url};
use parser::Package;
use std::process::exit;

#[tokio::main]
async fn main() -> Result<()> {
    let CliArgs {
        deps,
        owner,
        repo,
        branch,
        token,
    } = cli::CliArgs::parse();

    let github_url = get_project_api_url(&owner, &repo, &branch);

    println!("Parsing {}..", deps);

    let Package {
        mut dependencies,
        mut optional_dependencies,
        mut peer_dependencies,
        bundled_dependencies,
        ..
    } = Package::from_path(deps)?;

    let mut all_dependencies = types::DependencySet::new();

    // This is an array and needs to be merged in another way
    for name in bundled_dependencies {
        let key = name.to_string();
        all_dependencies.insert(key, "".to_string());
    }

    all_dependencies.append(&mut dependencies);
    all_dependencies.append(&mut optional_dependencies);
    all_dependencies.append(&mut peer_dependencies);

    println!("Get allowed dependencies..");

    let allowed_dependencies = get_approved_dependencies(github_url, token).await?;

    println!("Checking dependencies..\n");
    let results = compare::compare_licenses(all_dependencies, allowed_dependencies);

    if results.is_ok {
        println!("All good!");
        exit(0);
    }

    println!(
        "{}\nThere are some dependencies that need approval",
        results
    );
    exit(1);
}
