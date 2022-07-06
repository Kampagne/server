use std::{path::PathBuf, fs};
use sea_orm::{ Database, DatabaseConnection };
use migration::{ Migrator, MigratorTrait };
use dirs::home_dir;

pub mod models;

pub async fn start_connection( location: String ) -> DatabaseConnection {
    
    // Get path to home
    let mut path = PathBuf::new();
    if let Some( dir ) = home_dir() {
        path.push( dir );
        path.push( "Kampagne");
    }

    // Place files in the campaigns subdirectory
    path.push( "campaigns" );

    // Create all the necessary folders to create the path
    fs::create_dir_all( &path ).unwrap();

    // Add the file name
    path.push( location );

    // Get database URI and create the connection object
    let uri = format!( "sqlite:{}.db?mode=rwc", path.to_str().unwrap() );
    println!( "Database path: {}", uri );
    
    // Create database connection
    let connection = Database::connect( uri ).await.expect( "Failed to open database!" );

    // Run migrations on database
    Migrator::up( &connection, None ).await.expect( "Failed to run database migrations!" );

    // And return the connection
    connection
}