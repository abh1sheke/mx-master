use rust_xlsxwriter::{Format, Workbook};
use serde::{Deserialize, Serialize};

use crate::dns::MXRecord;

#[derive(Debug, Serialize, Deserialize)]
pub enum Error {
    SerdeErr,
    WriteErr,
    SaveErr,
}


impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match &self {
            Error::SerdeErr => write!(f, "Could deserialize input."),
            Error::WriteErr => write!(f, "Could not write row."),
            Error::SaveErr => write!(f, "Could not write file."),
        }
    }
}


#[tauri::command(async)]
pub async fn save_to(records: String, path: String) -> Result<String, Error> {
    let domains = serde_json::from_str::<Vec<MXRecord>>(&records).map_err(|_| Error::SerdeErr)?;
    let mut wb = Workbook::new();
    let wrap = Format::new().set_text_wrap();
    let sh = wb.add_worksheet();
    sh.write_with_format(0, 0, "domain", &wrap)
        .map_err(|_| Error::WriteErr)?;
    sh.write_with_format(0, 1, "ttl", &wrap)
        .map_err(|_| Error::WriteErr)?;
    sh.write_with_format(0, 2, "priority", &wrap)
        .map_err(|_| Error::WriteErr)?;
    sh.write_with_format(0, 3, "MX Target", &wrap)
        .map_err(|_| Error::WriteErr)?;
    for (row, data) in domains.iter().enumerate() {
        let row = (row + 1) as u32;
        sh.write_with_format(row, 0, &data.domain, &wrap)
            .map_err(|_| Error::WriteErr)?;
        sh.write_with_format(row, 1, data.ttl, &wrap)
            .map_err(|_| Error::WriteErr)?;
        sh.write_with_format(row, 2, data.priority, &wrap)
            .map_err(|_| Error::WriteErr)?;
        sh.write_with_format(row, 3, &data.target, &wrap)
            .map_err(|_| Error::WriteErr)?;
    }
    wb.save(path).map_err(|_| Error::SaveErr)?;
    return Ok("{\"success\": true}".into());
}
