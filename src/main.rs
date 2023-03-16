
use reqwest::Error;
use std::env;
use std::fs::File;
use std::io::Write;
use github_merged_prs_lib::fetch_merged_prs;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let repo_owner = "graphql";
    let access_token = env::var("GITHUB_ACCESS_TOKEN").expect("Missing GITHUB_ACCESS_TOKEN environment variable");

    let client = reqwest::Client::new();

    let repos = vec![
        "graphql-js",
        "graphql-spec",
        "graphiql",
        // Add more repository names as needed
    ];

    //let mut output_file = File::create("merged_prs.txt")?;

    for repo_name in repos {
        fetch_merged_prs(&repo_owner, &repo_name, &access_token, &client, &mut output_file).await?;
    }

    Ok(())
}

