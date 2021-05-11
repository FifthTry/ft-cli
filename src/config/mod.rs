pub mod env;
pub mod section;

pub struct Config {
    pub ignored: Vec<String>,
    pub repo: String,
    pub collection: String,
    pub backend: crate::Backend,
    pub root: String,
    pub mode: crate::SyncMode,
    pub auth: crate::Auth,
    pub dot_ft: bool,
    pub path: std::path::PathBuf,
}

impl Config {
    pub fn from_file(file_path: &str) -> crate::Result<Self> {
        use std::fs;
        let contents = fs::read_to_string(file_path)?;
        Self::parse(contents.as_str(), file_path)
    }

    pub fn parse(content: &str, file_path: &str) -> crate::Result<Self> {
        let p1 = ftd::p1::parse(content)?;
        let mut ft_sync: Option<section::FtSync> = None;
        let mut ignored: Vec<section::Ignored> = vec![];
        for section in p1 {
            let s = section::Section::from_p1(&section)?;
            match s {
                section::Section::FtSync(sec) => {
                    if ft_sync.is_none() {
                        ft_sync = Some(sec)
                    } else {
                        return Err(crate::Error::ConfigFileParseError {
                            error: "Duplicate ft-sync section".to_string(),
                        }
                        .into());
                    }
                }
                section::Section::Ignored(sec) => ignored.push(sec),
            }
        }

        let ft_sync = match ft_sync {
            Some(f) => f,
            None => {
                return Err(crate::Error::ConfigFileParseError {
                    error: "No FTSync section found".to_string(),
                }
                .into())
            }
        };

        let ignored = ignored
            .into_iter()
            .flat_map(|ig| ig.patterns)
            .collect::<Vec<_>>();

        Ok(Config {
            ignored,
            repo: ft_sync.repo,
            collection: ft_sync.collection,
            backend: ft_sync.backend.as_str().into(),
            root: ft_sync.root,
            mode: crate::SyncMode::LocalToRemote,
            auth: crate::Auth::AuthCode(crate::config::env::auth_code()),
            dot_ft: false,
            path: std::path::PathBuf::from(file_path),
        })
    }

    pub fn parent_dir(&self) -> std::path::PathBuf {
        let cwd = std::env::current_dir().unwrap();
        let config_file = self.path.as_path();
        config_file.parent().unwrap().join(cwd.as_path())
    }

    pub fn root_abs_path(&self) -> std::path::PathBuf {
        self.parent_dir()
            .join(std::path::Path::new(self.root.as_str()))
    }
}
