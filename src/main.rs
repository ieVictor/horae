use color_eyre::Result;
use color_eyre::eyre::Context;
use rusqlite::Connection;

mod app;
mod components;
mod db;
mod domain;
mod util;

fn main() -> Result<()> {
    color_eyre::install()?;

    let conn = Connection::open("horae.db")?;
    db::init(&conn)?;

    ratatui::run(|terminal| app::App::new(conn)?.run(terminal)).context("failed to run app")
}
