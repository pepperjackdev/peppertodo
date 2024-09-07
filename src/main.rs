use clap::Parser;
use peppertodo::{cli::Cli, manager::TaskManager, setup_application_directory};
use rusqlite::Connection;

fn main() {
    // Setting up the command line options
    let cli = Cli::parse();

    // Setting up the app's folder
    let app_home = setup_application_directory(env!("CARGO_PKG_NAME"));

    // Setting up the connection to the db
    let connection = Connection::open(app_home.join("appdata.db"))
        .expect("Unable to open or create the application's database");

    // Setting up the TaskManager
    let mut manager = TaskManager::new(&connection);

    if let Err(error) = peppertodo::run(&cli, &mut manager) {
        eprint!("Error: {error}");
    }
}
