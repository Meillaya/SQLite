use crate::storage::btree::BTreeTraverser;
use anyhow::{Result, anyhow, Context};
use std::fs::File;
use std::io::Read;

pub struct SqliteSchema {
    pub type_: String,
    pub name: String,
    pub tbl_name: String,
    pub rootpage: i32,
    pub sql: Option<String>,
}

impl BTreeTraverser {
    pub fn read_sqlite_schema(&mut self) -> Result<Vec<SqliteSchema>> {
        let root_page = 1; // SQLite schema is always on the first page
        let page = self.read_page(root_page).context("Failed to read root page")?;

        if page.page_type != 0x0D {
            return Err(anyhow!("Invalid root page type for sqlite_schema"));
        }

        let mut schema_entries = Vec::new();

        for cell in page.cells {
            let entry = self.parse_sqlite_schema_entry(&cell.payload.ok_or_else(|| anyhow!("Missing payload in sqlite_schema cell"))?)?;
            schema_entries.push(entry);
        }

        Ok(schema_entries)
    }

    fn parse_sqlite_schema_entry(&self, payload: &[u8]) -> Result<SqliteSchema> {
        let mut cursor = 0;

        let type_ = self.read_sqlite_string(payload, &mut cursor)?;
        let name = self.read_sqlite_string(payload, &mut cursor)?;
        let tbl_name = self.read_sqlite_string(payload, &mut cursor)?;
        let rootpage = self.read_sqlite_integer(payload, &mut cursor)?;
        let sql = self.read_sqlite_string(payload, &mut cursor).ok();

        Ok(SqliteSchema {
            type_,
            name,
            tbl_name,
            rootpage,
            sql,
        })
    }

    fn read_sqlite_string(&self, payload: &[u8], cursor: &mut usize) -> Result<String> {
        let length = self.read_sqlite_varint(payload, cursor)?;
        let end = *cursor + length as usize;
        if end > payload.len() {
            return Err(anyhow!("String length exceeds payload size"));
        }
        let s = String::from_utf8(payload[*cursor..end].to_vec())
            .context("Invalid UTF-8 in string")?;
        *cursor = end;
        Ok(s)
    }

    fn read_sqlite_integer(&self, payload: &[u8], cursor: &mut usize) -> Result<i32> {
        let value = self.read_sqlite_varint(payload, cursor)?;
        Ok(value as i32)
    }

    fn read_sqlite_varint(&self, payload: &[u8], cursor: &mut usize) -> Result<u64> {
        let mut result = 0u64;
        let mut shift = 0;

        loop {
            if *cursor >= payload.len() {
                return Err(anyhow!("Incomplete varint"));
            }
            let byte = payload[*cursor];
            *cursor += 1;

            result |= ((byte & 0x7F) as u64) << shift;
            if byte & 0x80 == 0 {
                break;
            }
            shift += 7;
            if shift >= 64 {
                return Err(anyhow!("Varint too large"));
            }
        }

        Ok(result)
    }
}

pub fn open_database(path: &str) -> Result<BTreeTraverser> {
    let mut file = File::open(path).context("Failed to open database file")?;
    let mut header = [0u8; 100];
    file.read_exact(&mut header).context("Failed to read database header")?;

    if &header[0..16] != b"SQLite format 3\0" {
        return Err(anyhow!("Not a valid SQLite database file"));
    }

    let page_size = u16::from_be_bytes([header[16], header[17]]);
    Ok(BTreeTraverser::new(file, page_size))
}
