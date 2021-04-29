#[derive(Debug, Default, Clone)]
pub struct FtSync {
    pub mode: String,
    pub backend: String,
    pub root: String,
    pub repo: String,
}


impl FtSync {
    pub fn from_p1(p1: &ftd::p1::Section) -> Result<Self, ftd::document::ParseError> {
        let mode = p1.header.string("mode")?;
        let backend = p1.header.string("backend")?;
        let root = p1.header.string("root")?;
        let repo = p1.header.string("repo")?;
        Ok(Self {
            mode, backend, root, repo
        })
    }
}
