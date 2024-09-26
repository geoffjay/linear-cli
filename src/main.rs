use clap::{Parser, Subcommand};
use dotenv::dotenv;
use graphql_client::{GraphQLQuery, Response};
use reqwest::blocking::Client;
use std::env;

mod queries;
use queries::IssuesQuery;

use crate::queries::issues_query;

#[derive(Parser)]
#[command(name = "linear")]
#[command(about = "Linear CLI for managing issues and pull requests", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Manage issues")]
    Issues {
        #[arg(short, long)]
        list: bool,
    },
}

fn main() {
    dotenv().ok();

    let cli = Cli::parse();

    match &cli.command {
        Commands::Issues { list } => {
            if *list {
                list_issues();
            } else {
                println!("No action specified for issues.");
            }
        }
    }
}

fn list_issues() {
    let api_token = env::var("LINEAR_API_TOKEN").expect("LINEAR_API_TOKEN must be set");
    let client = Client::new();
    let request_body = IssuesQuery::build_query(issues_query::Variables {});

    let res = client
        .post("https://api.linear.app/graphql")
        .header("Authorization", format!("{}", api_token))
        .json(&request_body)
        .send()
        .expect("Failed to send request");

    let response_body: Response<issues_query::ResponseData> = res.json().expect("Failed to parse response");

    if let Some(data) = response_body.data {
        for issue in data.issues.nodes {
            println!("{}: {}", issue.id, issue.title);
        }
    } else {
        println!("No issues found.");
    }
}