pub mod section;

pub struct Config {
    pub ignored: Vec<Ignored>,
    pub repo: String,
    pub collection: String,
    pub backend: crate::Backend,
    pub root: String,
    pub mode: crate::SyncMode,
    pub auth: crate::Auth,
    pub dot_ft: bool,
}

pub struct Ignored {
    pub pattern: String,
}

impl Config {
    pub fn from_file(filename: &str) -> crate::Result<Self> {
        use std::fs;
        let contents = fs::read_to_string(filename)?;
        Self::parse(contents.as_str())
    }

    pub fn parse(content: &str) -> crate::Result<Self> {
        let p1 = ftd::p1::parse(content)?;
        let mut ftsync: Option<section::FtSync> = None;
        let mut ignored: Vec<section::Ignored> = vec![];
        for section in p1 {
            let s = section::Section::from_p1(&section)?;
            match s {
                section::Section::FtSync(sec) => {
                    if ftsync.is_none() {
                        ftsync = Some(sec)
                    }
                    else {
                        return Err(crate::Error::ConfigFileParseError {error: "Duplicate FTSync section".to_string()}.into());
                    }
                },
                section::Section::Ignored(sec) => ignored.push(sec)
            }
        };

        let ftsync = match ftsync {
            Some(f) => f,
            None =>
                return Err(
                    crate::Error::ConfigFileParseError {error: "No FTSync section found".to_string()}.into()
                )
        };

        let patterns = ignored.into_iter().flat_map(|ig| ig.patterns).collect::<Vec<_>>();
        let patterns = patterns.into_iter().map(|x| Ignored{pattern: x}).collect();

        Ok(Config {
            ignored: patterns,
            repo: ftsync.repo,
            collection: ftsync.collection,
            backend: ftsync.backend.as_str().into(),
            root: ftsync.root,
            mode: crate::SyncMode::LocalToRemote,
            auth: crate::Auth::AuthCode("ZV6cN8i6B8VUrb5PgPKc".to_string()),
            dot_ft: false
        })
    }
}
