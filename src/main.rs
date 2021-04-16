mod configs;
mod errors;
mod helpers;
mod handlers;

use dotenv::dotenv;
use actix_web::{App, HttpServer};
use crate::handlers::index;
use crate::errors::AppErrors;
use crate::configs::get_configs;


#[actix_web::main]
async fn main() -> Result<(), AppErrors> {
    dotenv().ok();

    let configs = get_configs()?;
    let server = HttpServer::new(|| {
        App::new()
            .service(index)
    });

    server.bind(configs.get_app_addr())?
        .run()
        .await?;

    Ok(())
}
