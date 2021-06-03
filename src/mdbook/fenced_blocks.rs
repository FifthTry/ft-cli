// TODO: Need to discuss how can we improve this function and where should we keep different parts
pub(crate) fn fenced_to_code(content: &str) -> String {
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
    img_to_code(content.as_str())
}

pub(crate) fn img_to_code(content: &str) -> String {
    let mut sections = vec![];

    let mut buffer: String = "".to_string();
    for line in content.lines() {
        if line.starts_with("<img") && line.ends_with("/>") {
            if !buffer.is_empty() {
                let section = ftd::p1::Section::with_name("markdown")
                    .and_body(&buffer.drain(..).collect::<String>());
                sections.push(section.to_string())
            }

            let dom = html_parser::Dom::parse(content)
                .unwrap_or_else(|_| panic!("unable to parse: {}", line));

            if let Some(html_parser::Node::Element(element)) = dom.children.get(0) {
                if let Some(Some(src)) = element.attributes.get("src") {
                    let cap = if let Some(Some(alt)) = element.attributes.get("alt") {
                        alt.as_str()
                    } else {
                        ""
                    };
                    let section = ftd::p1::Section::with_name("image")
                        .add_header("src", src)
                        .and_caption(cap);
                    sections.push(section.to_string());
                }
            }
        } else {
            buffer.push_str(line);
            buffer.push('\n');
        }
    }

    sections.push(buffer.drain(..).collect());
    sections.join("\n")
}
