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
    for line in content.split('\n') {
        if line.trim().starts_with("```") && state.state == ParsingState::WaitingForBackTick {
            let lang = parse_lang(line);
            if !buffer.trim().eq("-- markdown:") {
                state.sections.push(buffer.drain(..).collect());
            } else {
                buffer.drain(..);
            }

            state.state = ParsingState::WaitingForEndBackTick;
            buffer = format!("-- code:\nlang: {}\n\n", lang);
        } else if line.trim().starts_with("```")
            && state.state == ParsingState::WaitingForEndBackTick
        {
            state.sections.push(buffer.drain(..).collect());
            state.state = ParsingState::WaitingForBackTick;
            buffer = "-- markdown:\n\n".to_string();
        } else {
            buffer.push_str(line);
            buffer.push('\n');
        }
    }

    state.sections.push(buffer.drain(..).collect());
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
