use async_graphql::{ MergedObject, Object, Schema, EmptySubscription, http::{ playground_source, GraphQLPlaygroundConfig }, Result, Error };
use poem::{ handler, IntoResponse, web::Html };

// Get in queries
use crate::database::models::ModelsQuery;

#[derive( Default )]
pub struct BaseQuery;
pub struct Mutation;

#[Object]
impl BaseQuery {
    /// Returns the sum of a and b
    async fn add( &self, a: i32, b: i32 ) -> i32 {
        a + b
    }

    /// Returns the result of subtracting 'b' from 'a'
    async fn subtract( &self, a: i32, b: i32 ) -> i32 {
        a - b
    }
}

#[derive(MergedObject, Default)]
pub struct Query( BaseQuery, ModelsQuery );

#[Object]
impl Mutation {
    async fn signup( &self, username: String, password: String ) -> Result<bool> {
        let _ = password;
        match username.as_str() {
            "Joey" => Ok( true ),
            _ => Err( Error::from( "Failed to create user" ) )
        }
    }
}

pub fn start_schema() -> async_graphql::SchemaBuilder<Query, Mutation, EmptySubscription> {
    return Schema::build( Query::default(), Mutation, EmptySubscription );
}

#[handler]
pub fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}