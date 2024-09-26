use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.json",
    query_path = "src/queries/issues_query.graphql",
    response_derives = "Debug"
)]
pub struct IssuesQuery;