use clap::{Parser, Subcommand};
use ngram::client::Client;
use ngram::server::Server;

// TODO:
// Fill out the `Args` struct to parse the command line arguments. You may find clap "subcommands"
// helpful.
/// An archive service allowing publishing and searching of books
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Start the client to send requests to the server
    Client {
        /// The server address to connect to (e.g., "127.0.0.1")
        server_address: String,
        
        /// The port number to connect to on the server
        server_port: u16,

        /// The client operation: publish, search, or retrieve
        #[command(subcommand)]
        operation: ClientCommand,
    },

    /// Start the server to listen for incoming client requests
    Server {
        /// The port number on which the server will listen
        listen_port: u16,
    },
}

#[derive(Subcommand, Debug)]
enum ClientCommand {
    /// Publish a document to the server
    Publish {
        /// Path to the document to publish
        path: String,
    },

    /// Search for a word in the server’s document archive
    Search {
        /// The word to search for
        word: String,
    },

    /// Retrieve a document from the server by its document ID
    Retrieve {
        /// The ID of the document to retrieve
        document_id: usize,
    },
}

// TODO:
// Inspect the contents of the `args` struct that has been created from the command line arguments
// the user passed. Depending on the arguments, either start a server or make a client and send the
// appropriate request. You may find it helpful to print the request response.
fn main() {
    let args = Args::parse();
    match args.command {
        Command::Client {
            server_address,
            server_port,
            operation,
        } => {
            match operation {
                ClientCommand::Publish { path } => {
                    println!(
                        "Connecting to {}:{} to publish document at path: {}",
                        server_address, server_port, path
                    );
                    // Call the publish function in Client (or use unimplemented!())
                    unimplemented!("Client publish functionality");
                }
                ClientCommand::Search { word } => {
                    println!(
                        "Connecting to {}:{} to search for word: {}",
                        server_address, server_port, word
                    );
                    // Call the search function in Client (or use unimplemented!())
                    unimplemented!("Client search functionality");
                }
                ClientCommand::Retrieve { document_id } => {
                    println!(
                        "Connecting to {}:{} to retrieve document with ID: {}",
                        server_address, server_port, document_id
                    );
                    // Call the retrieve function in Client (or use unimplemented!())
                    unimplemented!("Client retrieve functionality");
                }
            }
        }
        Command::Server { listen_port } => {
            println!("Starting server and listening on port: {}", listen_port);
            // Call the server start function (or use unimplemented!())
            unimplemented!("Server functionality");
        }
    }
}
