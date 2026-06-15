use iced::{
    widget::{column, container, scrollable, text, button},
    Element, Length,
};
use std::path::PathBuf;

use crate::app::Message;

#[derive(Debug, Clone)]
pub enum SidebarMessage {
    OpenFile(PathBuf),
    RemoveFile(usize),
}

pub struct SidebarState {
    pub files: Vec<PathBuf>,
    pub selected: Option<usize>,
}

impl SidebarState {
    pub fn new() -> Self {
        Self {
            files: Vec::new(),
            selected: None,
        }
    }

    pub fn add_file(&mut self, path: PathBuf) {
        if !self.files.contains(&path) {
            self.files.push(path);
        }
        self.selected = self.files.len().checked_sub(1);
    }

    pub fn update(&mut self, msg: SidebarMessage) -> Option<PathBuf> {
        match msg {
            SidebarMessage::OpenFile(path) => {
                self.selected = self.files.iter().position(|p| p == &path);
                Some(path)
            }
            SidebarMessage::RemoveFile(idx) => {
                if idx < self.files.len() {
                    self.files.remove(idx);
                    self.selected = None;
                }
                None
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let items: Vec<Element<Message>> = self
            .files
            .iter()
            .enumerate()
            .map(|(i, path)| {
                let name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("?")
                    .to_string();
                let _is_selected = self.selected == Some(i);
                let path_clone = path.clone();

                button(text(name).size(13))
                    .on_press(Message::Sidebar(SidebarMessage::OpenFile(path_clone)))
                    .width(Length::Fill)
                    .into()
            })
            .collect();

        let file_list = if items.is_empty() {
            column![text(t!("sidebar.no_files")).size(12)]
        } else {
            column(items).spacing(2)
        };

        container(
            scrollable(
                column![
                    text(t!("sidebar.files")).size(12),
                    file_list,
                ]
                .spacing(8)
                .padding(8)
            )
            .height(Length::Fill)
        )
        .width(200)
        .height(Length::Fill)
        .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_file_appends_and_deduplicates() {
        let mut sidebar = SidebarState::new();
        let path = PathBuf::from("/tmp/test.rs");
        sidebar.add_file(path.clone());
        sidebar.add_file(path.clone());
        assert_eq!(sidebar.files.len(), 1);
    }

    #[test]
    fn add_multiple_files() {
        let mut sidebar = SidebarState::new();
        sidebar.add_file(PathBuf::from("/tmp/a.rs"));
        sidebar.add_file(PathBuf::from("/tmp/b.py"));
        assert_eq!(sidebar.files.len(), 2);
        assert_eq!(sidebar.selected, Some(1));
    }

    #[test]
    fn remove_file() {
        let mut sidebar = SidebarState::new();
        sidebar.add_file(PathBuf::from("/tmp/a.rs"));
        sidebar.update(SidebarMessage::RemoveFile(0));
        assert!(sidebar.files.is_empty());
    }
}
