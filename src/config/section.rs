#[derive(Debug)]
pub enum Section {
    FtSync(FtSync),
    Ignored(Ignored),
}

impl Section {
    pub fn from_p1(p1: &ftd::p1::Section) -> Result<Self, ftd::p1::Error> {
        Ok(match p1.name.as_str() {
            "ft-sync" => Self::FtSync(FtSync::from_p1(p1)?),
            "ignored" => Self::Ignored(Ignored::from_p1(p1)?),
            t => {
                return Err(ftd::p1::Error::InvalidInput {
                    message: format!(
                        "unknown section {}, allowed sections are: 'ft-sync' and 'ignored'",
                        t
                    ),
                    context: p1.name.clone(),
                })
            }
        })
    }
}

#[derive(Debug, Default, Clone)]
pub struct Ignored {
    pub patterns: Vec<String>,
}

impl Ignored {
    pub fn from_p1(p1: &ftd::p1::Section) -> ftd::p1::Result<Self> {
        Ok(Self {
            patterns: if let Some(body) = p1.body.as_ref() {
                body.lines()
                    .into_iter()
                    .filter(|x| !x.trim().is_empty())
                    .map(|x| x.to_string())
                    .collect()
            } else {
                return Err(ftd::p1::Error::InvalidInput {
                    message: "body of ignore is empty".to_string(),
                    context: "todo".to_string(),
                });
            },
        })
    }
}

#[derive(Debug)]
pub struct FtSync {
    pub mode: String,
    pub backend: crate::Backend,
    pub root: String,
    pub repo: String,
    pub collection: String,
}

impl FtSync {
    pub fn from_p1(p1: &ftd::p1::Section) -> ftd::p1::Result<Self> {
        let mode = p1.header.string("mode")?;
        let backend = p1.header.str("backend")?;
        let backend = match crate::Backend::from(backend) {
            Some(v) => v,
            None => {
                return Err(ftd::p1::Error::InvalidInput {
                    message: "invalid backend (allowed: ftd)".to_string(),
                    context: backend.to_string(),
                })
            }
        };
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
