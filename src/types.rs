pub type Result<T> = std::result::Result<T, crate::Error>;

pub enum Auth {
    SignedIn(User),
    AuthCode(String),
    Anonymous,
}

pub struct User {
    pub cookie: String,
    pub username: String,
    pub name: String,
}

#[derive(Debug)]
#[allow(clippy::upper_case_acronyms)]
pub enum Backend {
    FTD,
    Raw,
}

impl Backend {
    pub fn from(s: &str) -> Option<Backend> {
        match s {
            "ftd" => Some(Backend::FTD),
            "raw" => Some(Backend::Raw),
            _ => None,
        }
    }

    pub fn is_raw(&self) -> bool {
        matches!(self, Backend::Raw)
    }
}

impl std::fmt::Display for Backend {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Backend::FTD => write!(f, "ftd"),
            Backend::Raw => write!(f, "raw"),
        }
    }
}

pub enum SyncMode {
    LocalToRemote,
    RemoteToLocal,
    TwoWay,
}

#[derive(Debug)]
pub enum FileMode {
    Deleted(String),
    Created(String),
    Modified(String),
}

impl FileMode {
    pub fn id(&self, root_dir: &str, collection: &str) -> String {
        let t = self
            .path()
            .strip_prefix(root_dir)
            .unwrap()
            .with_extension("")
            .to_str()
            .unwrap()
            .to_string();

        if t == "index" {
            collection.to_string()
        } else {
            collection.to_string() + "/" + t.as_str()
        }
    }

    pub fn id_with_extension(&self, root_dir: &str, collection: &str) -> String {
        let t = self
            .path()
            .strip_prefix(root_dir)
            .unwrap()
            .to_string_lossy()
            .to_string();

        if t == "index" {
            collection.to_string()
        } else {
            collection.to_string() + "/" + t.as_str()
        }
    }

    pub fn content(&self) -> crate::Result<String> {
        std::fs::read_to_string(self.path())
            .map_err(|e| crate::Error::ReadError(e, self.path_str()))
    }

    pub fn raw_content(&self) -> crate::Result<String> {
        let extension = self
            .path()
            .extension()
            .unwrap_or_else(|| {
                panic!(
                    "File extension not found: {}",
                    self.path().to_string_lossy()
                )
            })
            .to_string_lossy()
            .to_string();

        let section = if extension.eq("md") || extension.eq("mdx") {
            [
                ftd::Section::Heading(ftd::Heading::new(0, &self.path().to_string_lossy())),
                ftd::Section::Markdown(ftd::Markdown::from_body(self.content()?.as_str())),
            ]
        } else if extension.eq("rst") {
            [
                ftd::Section::Heading(ftd::Heading::new(0, &self.path().to_string_lossy())),
                ftd::Section::Rst(ftd::Rst::from_body(self.content()?.as_str())),
            ]
        } else {
            [
                ftd::Section::Heading(ftd::Heading::new(0, &self.path().to_string_lossy())),
                ftd::Section::Code(
                    ftd::Code::default()
                        .with_lang(&extension)
                        .with_code(self.content()?.as_str()),
                ),
            ]
        };

        Ok(ftd::p1::to_string(
            &section.iter().map(|v| v.to_p1()).collect::<Vec<_>>(),
        ))
    }

    pub fn path(&self) -> std::path::PathBuf {
        std::path::PathBuf::from(match self {
            FileMode::Created(v) => v,
            FileMode::Deleted(v) => v,
            FileMode::Modified(v) => v,
        })
    }

    pub fn path_str(&self) -> String {
        self.path().to_string_lossy().to_string()
    }

    pub fn extension(&self) -> String {
        self.path()
            .extension()
            .and_then(|v| v.to_str())
            .unwrap_or("")
            .to_lowercase()
    }
}
