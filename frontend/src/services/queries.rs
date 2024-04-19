use graphql_client::GraphQLQuery;



#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/all_users.graphql",
    response_derives = "Debug"
)]
struct AllUsers;
type ObjectId = String;


#[allow(unused)]
fn main() {
    let token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJlbWFpbCI6ImFzZmZhQGRzYWZhLmNvbSIsInVzZXJuYW1lIjoi5a-G56CBMTExIiwiZXhwIjoxMDAwMDAwMDAwMH0.NyEN13J5trkn9OlRqWv2xMHshysR9QPWclo_-q1cbF4y_9rbkpSI6ern-GgKIh_ED0Czk98M1fJ6tzLczbdptg";
    let build_query = AllUsers::build_query(all_users::Variables {
        token: token.to_string(),
    });
    let query = serde_json::json!(build_query);
}