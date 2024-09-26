#![allow(clippy::all, warnings)]
pub struct IssuesQuery;
pub mod issues_query {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "IssuesQuery";
    pub const QUERY: &str =
        "query IssuesQuery {\n  issues {\n    nodes {\n      id\n      title\n    }\n  }\n}";
    use super::*;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    #[derive(Serialize)]
    pub struct Variables;
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        pub issues: IssuesQueryIssues,
    }
    #[derive(Deserialize, Debug)]
    pub struct IssuesQueryIssues {
        pub nodes: Vec<IssuesQueryIssuesNodes>,
    }
    #[derive(Deserialize, Debug)]
    pub struct IssuesQueryIssuesNodes {
        pub id: ID,
        pub title: String,
    }
}
impl graphql_client::GraphQLQuery for IssuesQuery {
    type Variables = issues_query::Variables;
    type ResponseData = issues_query::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: issues_query::QUERY,
            operation_name: issues_query::OPERATION_NAME,
        }
    }
}
