use mongodb::bson::Document;
use mongodb::{
    bson::doc,
    options::{ClientOptions, ServerApi, ServerApiVersion},
    sync::{Client, Collection},
};

pub fn get_collection(uri: &str) -> mongodb::error::Result<Collection<Document>> {
    let mut client_options = ClientOptions::parse(uri).run()?;

    // Set the server_api field of the client_options object to Stable API version 1
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);
    client_options.app_name = Some("Camp CLI".to_string());

    // Create a new client and connect to the server
    let client = Client::with_options(client_options)?;

    // Send a ping to confirm a successful connection
    println!("Connecting to database...");
    client
        .database("freecodecamp")
        .run_command(doc! {"ping": 1})
        .run()?;
    println!("Successfully connected to database!");
    let db = client.database("freecodecamp");

    let collection = db.collection::<Document>("user");
    Ok(collection)
}
