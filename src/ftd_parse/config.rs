#[derive(Debug)]
pub struct Config {
    pub sections: Vec<crate::ftd_parse::section::Section>
}

impl Config {
    pub fn parse(content: &str) -> Result<Self, ftd::document::ParseError> {
        let p1 = ftd::p1::parse(content)?;
        let mut sections = vec![];
        for section in p1 {
            let s = crate::ftd_parse::section::Section::from_p1(&section)?;
            sections.push(s);
        }
        Ok(Self {
            sections
        })
    }
}