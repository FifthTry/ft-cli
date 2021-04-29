#[derive(Debug, Clone)]
pub enum Section {
    FtSync(crate::ftd_parse::ft_sync::FtSync),
    Ignored
}

impl Section {
    pub fn from_p1(p1: &ftd::p1::Section) -> Result<Self, ftd::document::ParseError> {
        Ok(match p1.name.as_str() {
            "ft-sync" => Self::FtSync(crate::ftd_parse::ft_sync::FtSync::from_p1(p1)?),
            t => {
                return Err(ftd::document::ParseError::ValidationError(format!(
                    "unknown section {}",
                    t
                )))
            }
        })
    }
}