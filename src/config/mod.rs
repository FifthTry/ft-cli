pub mod env;
pub mod section;

#[derive(Debug)]
pub struct Config {
    // https://www.fifthtry.com/fifthtry/ft-sync/config/
    pub ignored: Vec<String>,
    pub repo: String,
    pub collection: String,
    pub title: Option<String>,
    pub backend: crate::Backend,
    pub root: String,
    pub mode: crate::SyncMode,
    pub auth: crate::Auth,
    pub dot_ft: bool,
    pub path: std::path::PathBuf,
    pub index_extra: Vec<ftd::Section>,
    pub preserve_meta: bool,
}

impl Config {
    pub fn from_file(file_path: &str) -> crate::Result<Self> {
        use std::fs;
        let contents = fs::read_to_string(file_path)
            .map_err(|v| crate::Error::ReadError(v, file_path.to_string()))?;
        Self::parse(contents.as_str(), file_path)
    }

    pub fn parse(content: &str, file_path: &str) -> crate::Result<Self> {
        let p1 = ftd::p1::parse(content)?;
        let mut ft_sync: Option<section::FtSync> = None;
        let mut ignored: Vec<section::Ignored> = vec![];
        let mut index_extra: Option<section::IndexExtra> = None;
        for section in p1 {
            let s = section::Section::from_p1(&section)?;
            match s {
                section::Section::FtSync(sec) => {
                    if ft_sync.is_none() {
                        ft_sync = Some(sec)
                    } else {
                        return Err(crate::Error::ConfigFileParseError {
                            error: "Duplicate ft-sync section".to_string(),
                        });
                    }
                }
                section::Section::Ignored(sec) => ignored.push(sec),
                section::Section::IndexExtra(sec) => index_extra = Some(sec),
            }
        }

        let ft_sync = match ft_sync {
            Some(f) => f,
            None => {
                return Err(crate::Error::ConfigFileParseError {
                    error: "No FTSync section found".to_string(),
                })
            }
        };

        let ignored = ignored
            .into_iter()
            .flat_map(|ig| ig.patterns)
            .collect::<Vec<_>>();

        let index_extra = match index_extra {
            Some(f) => f.body.sections,
            None if ft_sync.backend.is_raw() || ft_sync.backend.is_mdbook() => {
                return Err(crate::Error::ConfigFileParseError {
                    error: "index-extra section not found in config".to_string(),
                })
            }
            _ => vec![],
        };

        Ok(Config {
            ignored,
            repo: ft_sync.repo,
            collection: ft_sync.collection,
            title: ft_sync.title,
            backend: ft_sync.backend,
            root: ft_sync.root,
            mode: crate::SyncMode::LocalToRemote,
            auth: crate::Auth::AuthCode(crate::config::env::auth_code()),
            dot_ft: false,
            path: std::path::PathBuf::from(file_path),
            index_extra,
            preserve_meta: ft_sync.preserve_meta,
        })
    }

    pub fn from_args(args: &clap::ArgMatches) -> Config {
        let repo = args
            .value_of("repo")
            .map(|x| x.to_string())
            .unwrap_or_else(|| panic!("repo is mandatory argument"));
        let collection = args
            .value_of("collection")
            .map(|x| x.to_string())
            .unwrap_or_else(|| panic!("collection is mandatory argument"));
        let root = args
            .value_of("root")
            .map(|x| x.to_string())
            .unwrap_or_else(|| "".to_string());
        let backend = args
            .value_of("backend")
            .map(|x| x.to_string())
            .unwrap_or_else(|| panic!("backend is mandatory argument"));

        let backend = match crate::Backend::from(&backend) {
            Some(v) => v,
            None => {
                panic!("invalid backend (allowed: ftd, mdbook, raw)")
            }
        };

        Config {
            ignored: vec![],
            repo,
            collection,
            title: None,
            backend,
            root,
            mode: crate::SyncMode::LocalToRemote,
            auth: crate::Auth::AuthCode(crate::config::env::auth_code()),
            dot_ft: false,
            path: std::path::PathBuf::from(""),
            index_extra: vec![],
            preserve_meta: true,
        }
    }

    pub fn parent_dir(&self) -> std::path::PathBuf {
        let cwd = std::env::current_dir().unwrap();
        let config_file = self.path.as_path();
        config_file.parent().unwrap().join(cwd.as_path())
    }

    pub fn root_path(&self) -> std::path::PathBuf {
        std::path::PathBuf::from(self.root.as_str())
    }
}
