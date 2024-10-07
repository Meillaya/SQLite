use anyhow::{Result, Context, bail};
use std::fs::File;
use std::io::Read;
use crate::db::connection::Connection;
use crate::storage::btree::BTreeTraverser;
use std::io::prelude::*;

mod db;
mod storage;
mod errors;

fn main() -> Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    match args.len() {
        0 | 1 => bail!("Missing <database path> and <command>"),
        2 => bail!("Missing <command>"),
        _ => {}
    }

    let command = &args[2];

    let mut file = File::open(&args[1]).context("Failed to open database file")?;
    let page_size = read_page_size(&mut file)?;

    let mut traverser = BTreeTraverser::new(file.try_clone().context("Failed to clone file handle")?, page_size);

    match command.as_str() {
        ".dbinfo" => {
            print_db_info(&mut traverser, page_size)?;
        }
        ".tables" => {
            list_tables(&mut traverser)?;
        }
        "select" => {
            // execute_select(&args[2..], &mut file)?;
        }
        _ => {
            anyhow::bail!("Unknown command: {}", command);
        }
    }

    Ok(())
}

fn read_page_size(file: &mut File) -> Result<u16> {
    let mut header = [0; 100];
    file.read_exact(&mut header).context("Failed to read database header")?;
    Ok(u16::from_be_bytes([header[16], header[17]]))
}

fn print_db_info(traverser: &mut BTreeTraverser, page_size: u16) -> Result<()> {
    println!("database page size: {}", page_size);

    let table_count = traverser.count_tables().context("Failed to count tables")?;
    println!("number of tables: {}", table_count);

    Ok(())
}

fn list_tables(traverser: &mut BTreeTraverser) -> Result<()> {
    let schema = traverser.read_sqlite_schema().context("Failed to read SQLite schema")?;

    for table in schema {
        if table.type_ == "table" {
            println!("{}", table.name);
        }
    }

    Ok(())
}

// fn execute_select(args: &[String], file: &mut File) -> Result<()> {
//     let mut connection = Connection::new(file.try_clone().context("Failed to clone file handle")?);
//     let sql = args.join(" ");
    
//     let rows = connection.query(&sql).context("Failed to execute query")?;

//     for row in rows {
//         println!("{}", row.join("|"));
//     }

//     Ok(())
// }
