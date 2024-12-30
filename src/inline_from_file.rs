use std::{fs::read_to_string, path::PathBuf};

use crate::CodePointToHtmlTag;

pub struct InlineFromFile {
    path: PathBuf,
}

impl InlineFromFile {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl CodePointToHtmlTag for InlineFromFile {
    fn code_point_to_html(&mut self, code_point: &str, emoji: &str) -> String {
        let mut file_path = self.path.clone();
        file_path.push(&code_point);
        file_path.set_extension("svg");

        let svg_content = read_to_string(file_path)
            .unwrap()
            .lines()
            .filter(|line| !line.trim().starts_with("<?xml"))
            .filter(|line| !line.trim().starts_with("<!--"))
            .collect::<Vec<_>>()
            .join("\n");

        let mut finished_tag = format!(
            "<svg class=\"emoji\" draggable=\"false\" style=\"{}\" alt=\"{}\"",
            crate::STYLES,
            emoji,
        );
        finished_tag.push_str(&svg_content[4..svg_content.len()]);

        finished_tag
    }
}
