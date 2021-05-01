#[derive(Debug, Clone)]
pub enum Section {
    FtSync(FtSync),
    Ignored(Ignored),
}

impl Section {
    pub fn from_p1(p1: &ftd::p1::Section) -> Result<Self, ftd::document::ParseError> {
        Ok(match p1.name.as_str() {
            "ft-sync" => Self::FtSync(FtSync::from_p1(p1)?),
            "ignored" => Self::Ignored(Ignored::from_p1(p1)?),
            t => {
                return Err(ftd::document::ParseError::ValidationError(format!(
                    "unknown section {}",
                    t
                )))
            }
        })
    }
}

#[derive(Debug, Default, Clone)]
pub struct Ignored {
    pub patterns: Vec<String>,
}

impl Ignored {
    pub fn from_p1(p1: &ftd::p1::Section) -> Result<Self, ftd::document::ParseError> {
        Ok(Self {
            patterns: if let Some(body) = p1.body.as_ref() {
                body.lines()
                    .into_iter()
                    .filter(|x| !x.trim().is_empty())
                    .map(|x| x.to_string())
                    .collect()
            } else {
                vec![]
            },
        })
    }
}

#[derive(Debug, Default, Clone)]
pub struct FtSync {
    pub mode: String,
    pub backend: String,
    pub root: String,
    pub repo: String,
    pub collection: String,
}

impl FtSync {
    pub fn from_p1(p1: &ftd::p1::Section) -> Result<Self, ftd::document::ParseError> {
        let mode = p1.header.string("mode")?;
        let backend = p1.header.string("backend")?;
        let root = p1.header.string("root")?;
        let repo = p1.header.string("repo")?;
        let collection = p1.header.string("collection")?;
        Ok(Self {
            mode,
            backend,
            root,
            repo,
            collection,
        })
    }
}
