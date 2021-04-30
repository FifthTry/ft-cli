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

    pub fn get_ft_sync(&self) -> Option<&crate::ftd_parse::ft_sync::FtSync> {
        for sec in self.sections.iter() {
            match sec {
                crate::ftd_parse::section::Section::FtSync(ft) => return Some(ft),
                _ => {}
            }
        }
        return None
    }

    pub fn get_ignored(&self) -> Option<&crate::ftd_parse::ignored::Ignored> {
        for sec in self.sections.iter() {
            match sec {
                crate::ftd_parse::section::Section::Ignored(ig) => return Some(ig),
                _ => {}
            }
        }
        return None
    }
}