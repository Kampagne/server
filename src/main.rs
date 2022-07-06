mod server;
mod parser;
mod database;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // let mut app_state = parser::ServerState { campaign_name: "".to_string() };

    match parser::handle_arguments() {
        Ok( state ) => {
            server::start_server( state ).await
        },
        Err( parser::ServerErrorTypes::Ignore ) => Ok(()),
        Err( _ ) => panic!( "Server has failed to start for unknown reasons!" )
    }
}