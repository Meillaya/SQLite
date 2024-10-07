use anyhow::{Result, anyhow};
use crate::db::connection::Connection;
use std::collections::HashMap;

pub struct Transaction<'a> {
    connection: &'a mut Connection,
    savepoints: HashMap<String, usize>,
    transaction_depth: usize,
}

impl<'a> Transaction<'a> {
    pub fn new(connection: &'a mut Connection) -> Self {
        Transaction {
            connection,
            savepoints: HashMap::new(),
            transaction_depth: 0,
        }
    }

    pub fn begin_transaction(&mut self) -> Result<()> {
        self.transaction_depth += 1;
        if self.transaction_depth == 1 {
            self.connection.execute("BEGIN TRANSACTION")?;
        }
        Ok(())
    }

    pub fn commit_transaction(&mut self) -> Result<()> {
        if self.transaction_depth == 0 {
            return Err(anyhow!("No active transaction to commit"));
        }
        self.transaction_depth -= 1;
        if self.transaction_depth == 0 {
            self.connection.execute("COMMIT")?;
        }
        Ok(())
    }

    pub fn rollback_transaction(&mut self) -> Result<()> {
        if self.transaction_depth == 0 {
            return Err(anyhow!("No active transaction to rollback"));
        }
        self.transaction_depth = 0;
        self.connection.execute("ROLLBACK")?;
        Ok(())
    }

    pub fn set_savepoint(&mut self, name: &str) -> Result<()> {
        self.savepoints.insert(name.to_string(), self.transaction_depth);
        self.connection.execute(&format!("SAVEPOINT {}", name))?;
        Ok(())
    }

    pub fn release_savepoint(&mut self, name: &str) -> Result<()> {
        if !self.savepoints.contains_key(name) {
            return Err(anyhow!("Savepoint {} does not exist", name));
        }
        self.savepoints.remove(name);
        self.connection.execute(&format!("RELEASE SAVEPOINT {}", name))?;
        Ok(())
    }

    pub fn rollback_to_savepoint(&mut self, name: &str) -> Result<()> {
        if let Some(&depth) = self.savepoints.get(name) {
            self.transaction_depth = depth;
            self.connection.execute(&format!("ROLLBACK TO SAVEPOINT {}", name))?;
            Ok(())
        } else {
            Err(anyhow!("Savepoint {} does not exist", name))
        }
    }

    pub fn in_transaction(&self) -> bool {
        self.transaction_depth > 0
    }

    pub fn get_transaction_depth(&self) -> usize {
        self.transaction_depth
    }
}
