//#[macro_use] extern crate rocket;
use std::path::{Path, PathBuf};
use rocket::fs::NamedFile;
use rocket::fs::FileServer;
use super::ipfs;
use rocket::response::status;

#[get("/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).await.ok()
}

#[get("/openexplorer")]
async fn explorer() -> Option<NamedFile> {
    NamedFile::open(Path::new("")).await.ok()
}

#[get("/data")]
async fn data() -> status::Accepted<String> {
    return status::Accepted(Some(ipfs::get_data().await));
}

#[get("/pins")]
async fn pins() -> status::Accepted<String> {
    return status::Accepted(Some(ipfs::get_pins().await));
}

#[rocket::main]
pub async fn rocket() {
    // rocket::build().mount("/", routes![index])
    rocket::build().mount("/", FileServer::from("public/"))
                     .mount("/", routes![data])
                     .mount("/", routes![pins])
                     .launch()
                     .await;
}
