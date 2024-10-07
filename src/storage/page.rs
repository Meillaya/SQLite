use std::io::{Error, ErrorKind};

pub struct Page {
    pub page_type: u8,
    pub cell_count: u16,
    pub cell_content_area: u16,
    pub right_most_pointer: Option<u32>,
    pub cells: Vec<u16>,
}

pub fn parse_page(page_data: &[u8]) -> Result<Page, Error> {
    if page_data.len() < 8 {
        return Err(Error::new(ErrorKind::InvalidData, "Page data too short"));
    }

    let page_type = page_data[0];
    let cell_count = u16::from_be_bytes([page_data[3], page_data[4]]);
    let cell_content_area = u16::from_be_bytes([page_data[5], page_data[6]]);

    let right_most_pointer = if page_type == 0x05 || page_type == 0x02 {
        if page_data.len() < 12 {
            return Err(Error::new(ErrorKind::InvalidData, "Interior page data too short"));
        }
        Some(u32::from_be_bytes([page_data[8], page_data[9], page_data[10], page_data[11]]))
    } else {
        None
    };

    let cell_pointer_array_offset = if right_most_pointer.is_some() { 12 } else { 8 };
    let mut cells = Vec::with_capacity(cell_count as usize);

    for i in 0..cell_count {
        let offset = cell_pointer_array_offset + i as usize * 2;
        if offset + 2 > page_data.len() {
            return Err(Error::new(ErrorKind::InvalidData, "Cell pointer array out of bounds"));
        }
        cells.push(u16::from_be_bytes([page_data[offset], page_data[offset + 1]]));
    }

    Ok(Page {
        page_type,
        cell_count,
        cell_content_area,
        right_most_pointer,
        cells,
    })
}
