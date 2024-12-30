use std::path::PathBuf;

use crate::CodePointToHtmlTag;

pub struct LinkFromUrl {
    path: PathBuf,
    extension: String,
}

impl LinkFromUrl {
    pub fn new(path: PathBuf, extension: String) -> Self {
        Self { path, extension }
    }
}

impl CodePointToHtmlTag for LinkFromUrl {
    fn code_point_to_html(&mut self, code_point: &str, emoji: &str) -> String {
        let mut file_url = self.path.clone();
        file_url.push(code_point);
        file_url.set_extension(&self.extension);

        format!(
            "<img class=\"emoji\" draggable=\"false\" style=\"{}\" alt=\"{}\" src=\"{}\">",
            crate::STYLES,
            emoji,
            file_url.display(),
        )
    }
}
