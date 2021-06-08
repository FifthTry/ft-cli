pub(crate) fn fenced_to_code(content: &str, img_src: &std::path::Path) -> String {
    #[derive(PartialEq)]
    enum ParsingState {
        WaitingForBackTick,
        WaitingForEndBackTick,
    }

    struct State {
        state: ParsingState,
        sections: Vec<String>,
    }

    let mut state = State {
        state: ParsingState::WaitingForBackTick,
        sections: vec![],
    };

    fn parse_lang(line: &str) -> String {
        let line = line.replace("```", "");
        let line = line.trim().split(',').collect::<Vec<_>>();
        (match line.get(0) {
            Some(&"rust") => "rs",
            Some(&"console") => "sh",
            Some(&"cmd") => "sh",
            Some(&"toml") => "toml",
            Some(&"java") => "java",
            Some(&"python") => "py",
            _ => "txt",
        })
        .to_string()
    }

    fn finalize(state: State) -> String {
        state.sections.join("\n")
    }

    let mut buffer: String = "".to_string();
    let mut is_markdown = false;
    let mut filename = Option::<String>::None;
    for line in content.split('\n') {
        if line.trim().starts_with("<span class=\"filename\"") && line.trim().ends_with("</span>") {
            let dom = html_parser::Dom::parse(line.trim()).unwrap();
            if let Some(html_parser::Node::Element(e)) = dom.children.get(0) {
                if let Some(html_parser::Node::Text(text)) = e.children.get(0) {
                    let text = if text.contains(':') {
                        match text.split(':').collect::<Vec<_>>().last() {
                            Some(s) => s.to_string(),
                            None => text.to_string(),
                        }
                    } else {
                        text.to_string()
                    };
                    filename = Some(text);
                }
            }
        } else if line.trim().starts_with("```") && state.state == ParsingState::WaitingForBackTick
        {
            let lang = parse_lang(line);
            if !buffer.is_empty() {
                let content = buffer.drain(..).collect::<String>();
                if !content.trim().is_empty() {
                    let section = if is_markdown {
                        ftd::Markdown::from_body(&content).to_p1().to_string()
                    } else {
                        content
                    };
                    state.sections.push(section);
                }
            }
            state.state = ParsingState::WaitingForEndBackTick;
            buffer = format!(
                "-- code:\nlang: {}{}\n\n",
                lang,
                filename
                    .take()
                    .map(|x| format!("\nfilename: {}", x))
                    .unwrap_or_else(|| "".to_string())
            );
            is_markdown = false;
        } else if line.trim().starts_with("```")
            && state.state == ParsingState::WaitingForEndBackTick
        {
            state.sections.push(buffer.drain(..).collect());
            state.state = ParsingState::WaitingForBackTick;
            is_markdown = true;
        } else {
            buffer.push_str(line);
            buffer.push('\n');
        }
    }

    if !buffer.is_empty() {
        let content = buffer.drain(..).collect::<String>();
        if !content.trim().is_empty() {
            let section = if is_markdown {
                ftd::Markdown::from_body(&content).to_p1().to_string()
            } else {
                content
            };
            state.sections.push(section);
        }
    }

    let content = finalize(state);
    // Need to remove this function call from here
    img_to_code(content.as_str(), img_src)
}

pub(crate) fn img_to_code(content: &str, img_src: &std::path::Path) -> String {
    let mut sections = vec![];
    let mut is_markdown = false;
    let mut buffer: String = "".to_string();
    for line in content.lines() {
        if line.starts_with("<img") && line.ends_with("/>") {
            if !buffer.is_empty() {
                let sec = if is_markdown {
                    ftd::Markdown::from_body(&buffer.drain(..).collect::<String>())
                        .to_p1()
                        .to_string()
                } else {
                    buffer.drain(..).collect::<String>()
                };
                sections.push(sec);
            }

            is_markdown = true;

            let dom = html_parser::Dom::parse(line)
                .unwrap_or_else(|_| panic!("unable to parse: {}", line));
            if let Some(html_parser::Node::Element(element)) = dom.children.get(0) {
                if let Some(Some(src)) = element.attributes.get("src") {
                    let cap = if let Some(Some(alt)) = element.attributes.get("alt") {
                        alt.as_str()
                    } else {
                        ""
                    };
                    let src = img_src.join(src);
                    let sec = ftd::Image::default()
                        .with_src(&src.to_string_lossy())
                        .with_caption(cap)
                        .with_width(500)
                        .with_alt(cap)
                        .to_p1()
                        .to_string();
                    sections.push(sec);
                }
            }
        } else {
            buffer.push_str(line);
            buffer.push('\n');
        }
    }

    if !buffer.is_empty() {
        let sec = if is_markdown {
            ftd::Markdown::from_body(&buffer.drain(..).collect::<String>())
                .to_p1()
                .to_string()
        } else {
            buffer.drain(..).collect::<String>()
        };
        sections.push(sec);
    }
    sections.join("\n\n")
}
