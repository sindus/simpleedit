pub mod highlighter;

use iced::{
    widget::{container, text_editor},
    Element, Length,
};
use iced::highlighter as hl;

use crate::{app::Message, config::Config};

pub struct EditorState {
    pub content: text_editor::Content,
    pub language: Option<String>,
}

impl EditorState {
    pub fn new() -> Self {
        Self {
            content: text_editor::Content::new(),
            language: None,
        }
    }

    pub fn from_content(text: &str) -> Self {
        Self {
            content: text_editor::Content::with_text(text),
            language: None,
        }
    }

    pub fn set_language_by_extension(&mut self, ext: &str) {
        self.language = extension_to_token(ext).map(|s| s.to_string());
    }

    pub fn view(&self, config: &Config) -> Element<Message> {
        let extension = self.language.clone().unwrap_or_default();
        let hl_theme = if config.dark_mode {
            hl::Theme::SolarizedDark
        } else {
            hl::Theme::InspiredGitHub
        };

        let editor = text_editor(&self.content)
            .on_action(Message::EditorAction)
            .highlight::<hl::Highlighter>(
                hl::Settings {
                    theme: hl_theme,
                    extension,
                },
                |highlight, _theme| highlight.to_format(),
            )
            .height(Length::Fill);

        container(editor)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

fn extension_to_token(ext: &str) -> Option<&'static str> {
    match ext {
        "rs" => Some("rs"),
        "py" => Some("py"),
        "js" | "jsx" => Some("js"),
        "ts" | "tsx" => Some("ts"),
        "html" | "htm" => Some("html"),
        "css" => Some("css"),
        "json" => Some("json"),
        "toml" => Some("toml"),
        "yaml" | "yml" => Some("yaml"),
        "md" => Some("md"),
        "sh" | "bash" => Some("sh"),
        "c" | "h" => Some("c"),
        "cpp" | "cc" | "cxx" | "hpp" => Some("cpp"),
        "java" => Some("java"),
        "go" => Some("go"),
        "rb" => Some("rb"),
        "php" => Some("php"),
        "swift" => Some("swift"),
        "kt" => Some("kt"),
        "xml" => Some("xml"),
        "sql" => Some("sql"),
        "lua" => Some("lua"),
        "r" => Some("r"),
        _ => None,
    }
}

pub mod toolbar {
    use iced::{
        widget::{button, row, text, Space},
        Element, Length,
    };

    use crate::app::Message;
    use crate::config::Config;

    pub fn view(config: &Config) -> Element<Message> {
        let dark_label = if config.dark_mode { "☀ Light" } else { "☾ Dark" };

        row![
            button(text(t!("toolbar.new").to_string()).size(13))
                .on_press(Message::NewFile)
                .padding([4, 10]),
            button(text(t!("toolbar.open").to_string()).size(13))
                .on_press(Message::OpenFile)
                .padding([4, 10]),
            button(text(t!("toolbar.save").to_string()).size(13))
                .on_press(Message::SaveFile)
                .padding([4, 10]),
            button(text(t!("toolbar.close").to_string()).size(13))
                .on_press(Message::CloseFile)
                .padding([4, 10]),
            Space::with_width(Length::Fill),
            button(text(t!("toolbar.search").to_string()).size(13))
                .on_press(Message::ToggleSearch)
                .padding([4, 10]),
            button(text(t!("toolbar.sidebar").to_string()).size(13))
                .on_press(Message::ToggleSidebar)
                .padding([4, 10]),
            button(text(dark_label).size(13))
                .on_press(Message::ThemeChanged(!config.dark_mode))
                .padding([4, 10]),
            button(text(t!("toolbar.preferences").to_string()).size(13))
                .on_press(Message::TogglePreferences)
                .padding([4, 10]),
        ]
        .spacing(4)
        .padding([6, 8])
        .into()
    }
}

pub mod statusbar {
    use iced::{
        widget::{row, text, Space},
        Element, Length,
    };

    use crate::app::Message;
    use std::path::PathBuf;

    pub fn view<'a>(
        status: &'a str,
        current_file: &'a Option<PathBuf>,
        is_dirty: bool,
    ) -> Element<'a, Message> {
        let file_info = current_file
            .as_ref()
            .and_then(|p| p.to_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| t!("status.no_file").to_string());

        let dirty_indicator = if is_dirty { " •" } else { "" };

        row![
            text(status).size(12),
            Space::with_width(Length::Fill),
            text(format!("{}{}", file_info, dirty_indicator)).size(12),
        ]
        .padding([4, 8])
        .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extension_to_token_rust() {
        assert_eq!(extension_to_token("rs"), Some("rs"));
    }

    #[test]
    fn extension_to_token_unknown() {
        assert_eq!(extension_to_token("xyz"), None);
    }

    #[test]
    fn extension_to_token_all_common() {
        let exts = ["py", "js", "ts", "html", "css", "json", "md", "sh", "c", "cpp"];
        for ext in exts {
            assert!(extension_to_token(ext).is_some(), "Missing token for .{}", ext);
        }
    }

    #[test]
    fn editor_state_new_is_empty() {
        let state = EditorState::new();
        // iced initializes Content::new() with a single newline
        assert!(state.content.text().trim().is_empty());
        assert!(state.language.is_none());
    }

    #[test]
    fn editor_state_from_content() {
        let state = EditorState::from_content("hello world");
        assert_eq!(state.content.text().trim(), "hello world");
    }

    #[test]
    fn set_language_by_extension_known() {
        let mut state = EditorState::new();
        state.set_language_by_extension("rs");
        assert_eq!(state.language, Some("rs".to_string()));
    }

    #[test]
    fn set_language_by_extension_unknown() {
        let mut state = EditorState::new();
        state.set_language_by_extension("unknown_ext");
        assert!(state.language.is_none());
    }
}
