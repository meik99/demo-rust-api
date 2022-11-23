#[macro_use] extern crate rocket;

mod api;

use std::sync::RwLock;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build().
    manage(api::state::DataHolder{
        data: RwLock::new(Vec::<api::entity::Object>::new()),
    }).
    manage(mongodb::Client::with_uri_str("mongodb://localhost:27017").await.expect("error initializing client")).
    mount("/", routes![api::object::index, api::object::post_object]).
    launch().await?;

    Ok(())
}
