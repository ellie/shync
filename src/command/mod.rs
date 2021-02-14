use eyre::Result;
use structopt::StructOpt;
use uuid::Uuid;

use crate::local::database::Sqlite;

mod history;
mod import;
mod server;

#[derive(StructOpt)]
pub enum AtuinCmd {
    #[structopt(
        about="manipulate shell history",
        aliases=&["h", "hi", "his", "hist", "histo", "histor"],
    )]
    History(history::Cmd),

    #[structopt(about = "import shell history from file")]
    Import(import::Cmd),

    #[structopt(about = "start an atuin server")]
    Server(server::Cmd),

    #[structopt(about = "generates a UUID")]
    Uuid,
}

impl AtuinCmd {
    pub fn run(self, db: &mut Sqlite) -> Result<()> {
        match self {
            AtuinCmd::History(history) => history.run(db),
            AtuinCmd::Import(import) => import.run(db),
            AtuinCmd::Server(server) => server.run(),

            AtuinCmd::Uuid => {
                println!("{}", Uuid::new_v4().to_simple().to_string());
                Ok(())
            }
        }
    }
}
