use chrono::{DateTime, Utc, Duration};
use reqwest::Error;
use serde_json::Value;
use std::env;

/// Get all merged PRs from the last week
/// 
/// Replace the repo_owner and repo_name variables with 
/// the organization name and the repository name.
/// 
/// Example:
/// let repo_owner = "graphql";
/// let repo_name = vec!["dataloader", 
///                 "graphql-spec", "graphql-js", "graphiql",
///                "graphql-over-http", ""];
#[tokio::main]
async fn main() -> Result<(), Error> {
    let repo_owner = "graphql"; // Replace with the repo owner's username
    let repo_name = "dataloader"; // Replace with the repository name
    let access_token = env::var("GITHUB_ACCESS_TOKEN").expect("Missing GITHUB_ACCESS_TOKEN environment variable");

    let client = reqwest::Client::new();
    let now = Utc::now();
    let one_week_ago = (now - Duration::weeks(1)).to_rfc3339();

    let prs_url = format!(
        "https://api.github.com/repos/{}/{}/pulls?state=closed&access_token={}&since={}",
        repo_owner, repo_name, access_token, one_week_ago
    );

    let prs_data: Value = client.get(&prs_url).send().await?.json().await?;

    writeln!(output_file, "Respository: {}/{}\n", repo_owner, repo_name).expect("Unable to write to file");

    for pr in prs_data.as_array().unwrap() {
        let merged = pr["merged"].as_bool().unwrap_or(false);

        if merged {
            let number = pr["number"].as_i64().unwrap();
            let title = pr["title"].as_str().unwrap();
            let user = pr["user"]["login"].as_str().unwrap();
            let created_at = pr["created_at"].as_str().unwrap();
            let updated_at = pr["updated_at"].as_str().unwrap();
            let merged_at = pr["merged_at"].as_str().unwrap();

            writeln!(output_file, "PR #{}: {}\nAuthor: {}\nCreated at: {}\nUpdated at: {}\nMerged at: {}\n",
                number, title, user, created_at, updated_at, merged_at
            )?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_prs() {
        let result = main().await;
        assert!(result.is_ok());
    }
}

