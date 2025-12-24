use graphql_client::{GraphQLQuery, Response};
use reqwest::{Client, header};
use std::{env, error::Error};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/repo_query.graphql",
    response_derives = "Debug"
)]
pub struct RepoView;

const GITHUB_API_URL: &str = "https://api.github.com/graphql";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let github_token = env::var("GITHUB_TOKEN").unwrap();

    let headers = header::HeaderMap::from_iter([
        (
            header::AUTHORIZATION,
            format!("Bearer {github_token}").parse::<header::HeaderValue>()?,
        ),
        (header::USER_AGENT, "rust-graphql-client".parse()?),
    ]);
    let client = Client::builder().default_headers(headers).build()?;

    let variables = repo_view::Variables {
        owner: "seri".to_string(),
        name: "gettc".to_string(),
    };
    let request_body = RepoView::build_query(variables);
    let response = client
        .post(GITHUB_API_URL)
        .json(&request_body)
        .send()
        .await?;
    let response_body: Response<repo_view::ResponseData> = response.json().await?;

    if let Some(data) = &response_body.data {
        println!("{:#?}", data);
    }

    Ok(())
}
