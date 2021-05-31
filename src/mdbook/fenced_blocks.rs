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
    finalize(state)
}
