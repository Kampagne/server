use std::env;

// use juniper::FromInputValue;

fn get_args() -> Vec<String> {
    env::args().collect()
}

// pub fn list_arguments() {
//     let args = get_args();

//     for arg in args {
//         println!( "Received argument: '{}'", arg )
//     }
// }

pub struct ServerState {
    pub campaign_name: String,
}

pub enum ServerErrorTypes {
    MissingName,
    // Other,
    Ignore,
}

fn print_help_text() {
    println!( "server.exe [-h] campaign_name" );
    println!( "This executable is the server component of Kampagne! To run, it" );
    println!( "requires the name of the campaign you would like to serve. For" );
    println!( "example:" );
    println!( "   ./server MyWorld" );
    println!( "" );
    println!( "The server can only host one campaign at a time!" );
}

pub fn handle_arguments() -> Result<ServerState, ServerErrorTypes> {
    let args = get_args();

    if args.contains( &String::from( "-h" ) ) || args.contains( &String::from( "--help" ) ) {
        // return ServerState{ campaign_name: "".to_string() }
        // return Err( ServerState{ campaign_name: "".to_string() } )
        print_help_text();
        return Err( ServerErrorTypes::Ignore )
    }

    if args.get( 1 ).is_none() {
        return Err( ServerErrorTypes::MissingName )
    }
    
    Ok(
        ServerState { 
            campaign_name: args.get( 1 ).unwrap().to_string(),
        }
    )
}