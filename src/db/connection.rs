use crate::storage::btree::{BTreeTraverser, BTree};
use crate::db::query::{Query, parse};
use std::path::Path;
use std::fs::File;
use anyhow::{Context, Result, anyhow};

pub struct Connection {
    btree: BTree,
    last_insert_rowid: i64,
}

impl Connection {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = File::open(path).context("Failed to open database file")?;
        let page_size = 4096; // You might want to read this from the database header
        let btree_traverser = BTreeTraverser::new(file, page_size);
        let btree = BTree::new(btree_traverser);
        Ok(Connection { btree, last_insert_rowid: 0 })
    }

    pub fn execute(&mut self, sql: &str) -> Result<()> {
        let query = parse(sql).context("Failed to parse SQL query")?;
        match query {
            Query::Insert(insert_query) => {
                // Implement insert logic
                self.last_insert_rowid = 0; // Update this with the actual last insert ID
                Ok(())
            },
            Query::Update(update_query) => {
                // Implement update logic
                Ok(())
            },
            Query::Delete(delete_query) => {
                // Implement delete logic
                Ok(())
            },
            _ => Err(anyhow!("Unsupported query type for execute")),
        }
    }

    pub fn query(&self, sql: &str) -> Result<Vec<Vec<String>>> {
        let query = parse(sql).context("Failed to parse SQL query")?;
        match query {
            Query::Select(select_query) => {
                // Implement select logic
                Ok(vec![]) // Replace with actual results
            },
            _ => Err(anyhow!("Unsupported query type for query")),
        }
    }

    pub fn get_last_insert_rowid(&self) -> i64 {
        self.last_insert_rowid
    }
}

pub struct PreparedStatement {
    // Add fields as needed
}

impl PreparedStatement {
    pub fn execute(&self) -> Result<()> {
        // Implement prepared statement execution
        Err(anyhow!("Prepared statement execution not yet implemented"))
    }
}
