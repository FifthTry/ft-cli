#[derive(Debug, Default, Clone)]
pub struct Ignored {
    pub patterns: Vec<String>
}

impl Ignored {
    pub fn from_p1(p1: &ftd::p1::Section) -> Result<Self, ftd::document::ParseError> {
        Ok(Self {
            patterns: if let Some(body) = p1.body.as_ref() {
            body.lines().into_iter()
                .filter(|x| !x.trim().is_empty())
                .map(|x| x.to_string())
                .collect()
        } else {
            vec![]
        }})
    }
}