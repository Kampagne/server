// External packages
use poem::{get, listener::TcpListener, Route, Server, endpoint::StaticFilesEndpoint};
use async_graphql_poem::{  GraphQL };

// Server modules
mod api;
use crate::parser::ServerState;
use crate::database;

fn get_available_port( starting_port: u16, ending_port: u16 ) -> Option<u16> {
    (starting_port..ending_port).find(|port| {
        match std::net::TcpListener::bind(("127.0.0.1", *port)) {
            Ok(_) => true,
            Err(_) => false,
        }
    })
}

pub async fn start_server( state: ServerState ) -> Result<(), std::io::Error> {
    let db = database::start_connection( state.campaign_name ).await;
    let schema = api::start_schema().data( db ).finish();

    let port = get_available_port(3000, 4000).expect( "Could not start server! No open ports found between 3000 and 4000!" );
    println!( "Starting Kampagne-Server on localhost:{}", port );

    let app = Route::new()
        // .at("/hello/:name", get(hello))
        .at( "/graphql", get(api::graphql_playground).post(GraphQL::new( schema )))
        .nest("/", StaticFilesEndpoint::new( "./submodules/interface/dist" ).index_file("index.html") );

    Server::new(TcpListener::bind(("0.0.0.0", port)))
        .run(app)
        .await
}