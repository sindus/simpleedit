use iced::{
    executor,
    widget::{column, container, row, text_editor},
    Application, Command, Element, Length, Theme,
};
use std::path::PathBuf;

use crate::{
    config::Config,
    editor::EditorState,
    preferences::{PreferencesMessage, PreferencesState},
    search::{SearchMessage, SearchState},
    sidebar::{SidebarMessage, SidebarState},
};

#[derive(Debug, Clone)]
pub enum Message {
    EditorAction(text_editor::Action),
    Sidebar(SidebarMessage),
    Search(SearchMessage),
    Preferences(PreferencesMessage),
    OpenFile,
    FileOpened(Result<(PathBuf, String), String>),
    SaveFile,
    FileSaved(Result<PathBuf, String>),
    NewFile,
    CloseFile,
    ToggleSearch,
    ToggleSidebar,
    TogglePreferences,
    ThemeChanged(bool), // false = light, true = dark
}

pub struct TinctaApp {
    config: Config,
    editor: EditorState,
    sidebar: SidebarState,
    search: SearchState,
    preferences: PreferencesState,
    show_search: bool,
    show_sidebar: bool,
    show_preferences: bool,
    current_file: Option<PathBuf>,
    is_dirty: bool,
    status_message: String,
}

impl Application for TinctaApp {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let config = Config::load();
        let editor = EditorState::new();

        (
            Self {
                editor,
                sidebar: SidebarState::new(),
                search: SearchState::new(),
                preferences: PreferencesState::from_config(&config),
                show_search: false,
                show_sidebar: true,
                show_preferences: false,
                current_file: None,
                is_dirty: false,
                status_message: t!("status.ready").to_string(),
                config,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        let untitled = t!("app.untitled").to_string();
        let file_name = self
            .current_file
            .as_ref()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .unwrap_or(untitled.as_str());
        let dirty = if self.is_dirty { " •" } else { "" };
        format!("Tincta — {}{}", file_name, dirty)
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::EditorAction(action) => {
                let is_edit = action.is_edit();
                self.editor.content.perform(action);
                if is_edit {
                    self.is_dirty = true;
                }
                Command::none()
            }
            Message::NewFile => {
                self.editor = EditorState::new();
                self.current_file = None;
                self.is_dirty = false;
                self.status_message = t!("status.new_file").to_string();
                Command::none()
            }
            Message::OpenFile => Command::perform(open_file(), Message::FileOpened),
            Message::FileOpened(result) => {
                match result {
                    Ok((path, content)) => {
                        self.editor = EditorState::from_content(&content);
                        let ext = path
                            .extension()
                            .and_then(|e| e.to_str())
                            .unwrap_or("")
                            .to_string();
                        self.editor.set_language_by_extension(&ext);
                        self.sidebar.add_file(path.clone());
                        self.current_file = Some(path);
                        self.is_dirty = false;
                        self.status_message = t!("status.file_opened").to_string();
                    }
                    Err(e) => {
                        self.status_message = format!("{}: {}", t!("status.error"), e);
                    }
                }
                Command::none()
            }
            Message::SaveFile => {
                if let Some(ref path) = self.current_file.clone() {
                    let content = self.editor.content.text().to_string();
                    let path = path.clone();
                    Command::perform(save_file(path, content), Message::FileSaved)
                } else {
                    Command::perform(
                        save_file_as(self.editor.content.text().to_string()),
                        Message::FileSaved,
                    )
                }
            }
            Message::FileSaved(result) => {
                match result {
                    Ok(path) => {
                        self.current_file = Some(path);
                        self.is_dirty = false;
                        self.status_message = t!("status.file_saved").to_string();
                    }
                    Err(e) => {
                        self.status_message = format!("{}: {}", t!("status.error"), e);
                    }
                }
                Command::none()
            }
            Message::CloseFile => {
                self.editor = EditorState::new();
                self.current_file = None;
                self.is_dirty = false;
                self.status_message = t!("status.ready").to_string();
                Command::none()
            }
            Message::ToggleSearch => {
                self.show_search = !self.show_search;
                Command::none()
            }
            Message::ToggleSidebar => {
                self.show_sidebar = !self.show_sidebar;
                Command::none()
            }
            Message::TogglePreferences => {
                self.show_preferences = !self.show_preferences;
                Command::none()
            }
            Message::ThemeChanged(dark) => {
                self.config.dark_mode = dark;
                self.config.save();
                Command::none()
            }
            Message::Sidebar(msg) => {
                if let Some(path) = self.sidebar.update(msg) {
                    return Command::perform(read_file(path), Message::FileOpened);
                }
                Command::none()
            }
            Message::Search(msg) => {
                self.search.update(msg, &mut self.editor.content);
                Command::none()
            }
            Message::Preferences(msg) => {
                self.preferences.update(msg, &mut self.config);
                self.config.save();
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        // Toolbar
        let toolbar = crate::editor::toolbar::view(&self.config);

        // Editor
        let editor_widget = self.editor.view(&self.config);

        // Search panel (conditionally shown)
        let search_panel = if self.show_search {
            Some(self.search.view())
        } else {
            None
        };

        // Main content area
        let editor_area: Element<Message> = if let Some(search) = search_panel {
            column![search, editor_widget].into()
        } else {
            editor_widget.into()
        };

        // Sidebar (conditionally shown)
        let main_content: Element<Message> = if self.show_sidebar {
            row![self.sidebar.view(), editor_area,].into()
        } else {
            editor_area
        };

        // Status bar
        let status_bar =
            crate::editor::statusbar::view(&self.status_message, &self.current_file, self.is_dirty);

        let content = column![toolbar, main_content, status_bar,]
            .width(Length::Fill)
            .height(Length::Fill);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn theme(&self) -> Theme {
        if self.config.dark_mode {
            Theme::Dark
        } else {
            Theme::Light
        }
    }
}

async fn open_file() -> Result<(PathBuf, String), String> {
    let handle = rfd::AsyncFileDialog::new()
        .set_title("Open File")
        .pick_file()
        .await
        .ok_or_else(|| "cancelled".to_string())?;

    let path = handle.path().to_path_buf();
    let content =
        std::fs::read_to_string(&path).map_err(|e| e.to_string())?;

    Ok((path, content))
}

async fn read_file(path: PathBuf) -> Result<(PathBuf, String), String> {
    let content = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
    Ok((path, content))
}

async fn save_file(path: PathBuf, content: String) -> Result<PathBuf, String> {
    std::fs::write(&path, &content).map_err(|e| e.to_string())?;
    Ok(path)
}

async fn save_file_as(content: String) -> Result<PathBuf, String> {
    let handle = rfd::AsyncFileDialog::new()
        .set_title("Save File As")
        .save_file()
        .await
        .ok_or_else(|| "cancelled".to_string())?;

    let path = handle.path().to_path_buf();
    std::fs::write(&path, &content).map_err(|e| e.to_string())?;
    Ok(path)
}
