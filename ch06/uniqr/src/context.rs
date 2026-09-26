pub enum UniqrContext {
    Start,
    NewLine { line: String },
    SameLine { line: String, count: usize },
}

impl UniqrContext {
    pub fn new() -> UniqrContext {
        UniqrContext::Start
    }

    pub fn add_line(self, new_line: &str) -> Self {
        let new_line = new_line.trim_end();
        match self {
            UniqrContext::Start => UniqrContext::NewLine {
                line: new_line.to_string(),
            },
            UniqrContext::NewLine { line } => {
                if line == new_line {
                    UniqrContext::SameLine { line, count: 2 }
                } else {
                    UniqrContext::NewLine {
                        line: new_line.to_string(),
                    }
                }
            }
            UniqrContext::SameLine { line, count } => {
                if line == new_line {
                    UniqrContext::SameLine {
                        line,
                        count: count + 1,
                    }
                } else {
                    UniqrContext::NewLine {
                        line: new_line.to_string(),
                    }
                }
            }
        }
    }

    pub fn count(&self) -> usize {
        match self {
            UniqrContext::Start => 0,
            UniqrContext::SameLine { count, .. } => *count,
            _ => 1,
        }
    }
}
