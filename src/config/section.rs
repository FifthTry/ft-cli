#[derive(Debug)]
pub enum Section {
    FtSync(FtSync),
    Ignored(Ignored),
    IndexExtra(IndexExtra),
}

impl Section {
    pub fn from_p1(p1: &ftd::p1::Section) -> Result<Self, ftd::p1::Error> {
        Ok(match p1.name.as_str() {
            "ft-sync" => Self::FtSync(FtSync::from_p1(p1)?),
            "ignored" => Self::Ignored(Ignored::from_p1(p1)?),
            "index-extra" => Self::IndexExtra(IndexExtra::from_index_extra(p1)?),
            "meta" => Self::IndexExtra(IndexExtra::from_meta(p1)?),
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
    pub title: Option<String>,
    pub preserve_meta: bool,
}

impl FtSync {
    pub fn from_p1(p1: &ftd::p1::Section) -> ftd::p1::Result<Self> {
        Ok(Self {
            mode: p1.header.string("mode")?,
            backend: {
                let b = p1.header.str("backend")?;
                match crate::Backend::from(b) {
                    Some(v) => v,
                    None => {
                        return Err(ftd::p1::Error::InvalidInput {
                            message: "invalid backend (allowed: ftd, mdbook, raw)".to_string(),
                            context: b.to_string(),
                        })
                    }
                }
            },
            root: p1
                .header
                .string_optional("root")?
                .unwrap_or_else(|| "".to_string()), // Empty because it is relative to git root
            repo: p1.header.string("repo")?,
            collection: p1.header.string("collection")?,
            title: p1.header.string_optional("title")?,
            preserve_meta: p1.header.bool_with_default("preserve-meta", false)?,
        })
    }
}

#[derive(Debug)]
pub struct IndexExtra {
    pub body: ftd::Document,
}

impl IndexExtra {
    pub fn from_index_extra(p1: &ftd::p1::Section) -> ftd::p1::Result<Self> {
        Ok(Self {
            body: match p1.body.as_ref() {
                Some(b) => ftd::Document::parse(b, "ft-sync").map_err(|e| {
                    ftd::p1::Error::InvalidInput {
                        message: "Can not parse index-extra".to_string(),
                        context: e.to_string(),
                    }
                })?,
                None => {
                    return Err(ftd::p1::Error::InvalidInput {
                        message: "body of index-extra section is empty".to_string(),
                        context: "".to_string(),
                    })
                }
            },
        })
    }

    pub fn from_meta(p1: &ftd::p1::Section) -> ftd::p1::Result<Self> {
        Ok(Self {
            body: ftd::Document::new(&vec![ftd::Section::Meta(ftd::Meta::from_p1(p1).map_err(
                |e| ftd::p1::Error::InvalidInput {
                    message: "Can not parse index-extra".to_string(),
                    context: e.to_string(),
                },
            )?)]),
        })
    }
}
