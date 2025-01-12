// TODO: I tested this on my site, but it needs actual tests.
mod inline_from_file;
mod link_from_url;

use std::path::PathBuf;

pub use inline_from_file::InlineFromFile;
pub use link_from_url::LinkFromUrl;

pub(crate) const STYLES: &str =
    "height: 1em; width: 1em; margin: 0 .05em 0 .1em; vertical-align: -0.1em;";

pub trait CodePointToHtmlTag {
    fn code_point_to_html(&mut self, code_point: &str, emoji: &str) -> String;
}

fn is_emoji_start(ch: &char) -> bool {
    matches!(ch,
        '\u{1F300}'..='\u{1F9FF}' |
        '\u{2600}'..='\u{26FF}' |
        // '\u{2700}'..='\u{27BF}' |
        '\u{1F100}'..='\u{1F1FF}' |
        '\u{1F200}'..='\u{1F2FF}'
    )
}

fn is_emoji_modifier(ch: &char) -> bool {
    matches!(
        ch,
        '\u{1F3FB}'..='\u{1F3FF}' | '\u{FE0F}' | '\u{20E3}' | '\u{200D}'
    )
}

fn is_emoji_component(ch: &char) -> bool {
    is_emoji_start(ch) || is_emoji_modifier(ch)
}

pub struct TwemojiParser<T: CodePointToHtmlTag> {
    code_point_to_html_tag: T,
}

impl TwemojiParser<InlineFromFile> {
    pub fn inline_from_local_file(path: PathBuf) -> Self {
        Self::new(InlineFromFile::new(path))
    }
}

impl TwemojiParser<LinkFromUrl> {
    pub fn link_from_url(path: PathBuf, extension: String) -> Self {
        Self::new(LinkFromUrl::new(path, extension))
    }
}

impl<T: CodePointToHtmlTag> TwemojiParser<T> {
    pub fn new(code_point_to_html_tag: T) -> Self {
        Self {
            code_point_to_html_tag,
        }
    }

    pub fn parse(&mut self, text: &str) -> String {
        let mut result = String::with_capacity(text.len());
        let mut characters = text.chars().peekable();
        let mut emoji = String::new();

        while let Some(character) = characters.next() {
            if !is_emoji_start(&character) {
                result.push(character);
                continue;
            }

            emoji.clear();
            emoji.push(character);

            while let Some(&next_character) = characters.peek() {
                if next_character == '\u{FE0F}' {
                    let _ = characters.next();
                    continue;
                }

                if is_emoji_modifier(&next_character) {
                    emoji.push(characters.next().unwrap());
                } else if next_character == '\u{200D}' {
                    // Look ahead one more character after the ZWJ
                    emoji.push(characters.next().unwrap());
                    if let Some(&following_char) = characters.peek() {
                        if is_emoji_component(&following_char) {
                            emoji.push(characters.next().unwrap());
                            continue;
                        }
                    }
                }
                break;
            }

            if !emoji.is_empty() {
                let code_point = emoji
                    .chars()
                    .map(|c| format!("{:x}", c as u32))
                    .collect::<Vec<_>>()
                    .join("-");

                result.push_str(
                    &self
                        .code_point_to_html_tag
                        .code_point_to_html(&code_point, &emoji),
                );
            }
        }

        result
    }
}
