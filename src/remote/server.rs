use std::collections::HashMap;

use crate::remote::database::establish_connection;
use crate::settings::Settings;

use super::database::AtuinDbConn;

use eyre::Result;
use rocket::config::{Config, Environment, LoggingLevel, Value};

// a bunch of these imports are generated by macros, it's easier to wildcard
#[allow(clippy::clippy::wildcard_imports)]
use super::views::*;

#[allow(clippy::clippy::wildcard_imports)]
use super::auth::*;

embed_migrations!("migrations");

pub fn launch(settings: &Settings, host: String, port: u16) -> Result<()> {
    let settings: Settings = settings.clone(); // clone so rocket can manage it

    let mut database_config = HashMap::new();
    let mut databases = HashMap::new();

    database_config.insert("url", Value::from(settings.server.db_uri.clone()));
    databases.insert("atuin", Value::from(database_config));

    let connection = establish_connection(&settings)?;

    embedded_migrations::run(&connection).expect("failed to run migrations");

    let config = Config::build(Environment::Production)
        .address(host)
        .log_level(LoggingLevel::Normal)
        .port(port)
        .extra("databases", databases)
        .finalize()
        .unwrap();

    let app = rocket::custom(config);

    app.mount(
        "/",
        routes![
            index,
            register,
            add_history,
            login,
            get_user,
            sync_count,
            sync_list
        ],
    )
    .manage(settings)
    .attach(AtuinDbConn::fairing())
    .register(catchers![internal_error, bad_request])
    .launch();

    Ok(())
}
