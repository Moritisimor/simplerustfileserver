use std::io;

use actix_files::Files;
use actix_web::{App, HttpServer};
use clap::Parser;

use crate::structs::Config;

mod structs;

#[actix_web::main]
async fn main() -> io::Result<()> {
    let args = Config::parse();
    args.tell_about_self();

    HttpServer::new(move || {
        App::new()
        .service(Files::new(&args.url_path, &args.shared_dir)
            .show_files_listing()
            .index_file(&args.index_file))
    })
    .bind(args.server_address)?
    .run()
    .await
}
