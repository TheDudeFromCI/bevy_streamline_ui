//! Contains components, systems, and behaviors for handling input within text
//! fields.

use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::input::ButtonState;
use bevy::prelude::*;

/// A shared handle for the default font to use for cursors within text fields.
pub const CURSOR_HANDLE: Handle<Font> = Handle::weak_from_u128(10482756907980398621);

/// A component that represents a text field.
#[derive(Debug, Component)]
pub struct TextField {
    /// The text currently in the field.
    pub text: String,

    /// The current position of the cursor.
    pub cursor_pos: usize,

    /// The timer for the cursor blink.
    pub cursor_blink_timer: Timer,

    /// Whether or not the cursor is currently shown.
    ///
    /// This is used by the timer to make the cursor blink.
    pub cursor_blink: bool,

    /// Whether or not the text field is currently active.
    pub active: bool,

    /// The currently selection region of text, if any.
    pub selection: Option<TextSelection>,

    /// The font to use for the text.
    pub font: Handle<Font>,

    /// The size of the text.
    pub font_size: f32,

    /// The color of the text.
    pub font_color: Color,

    /// The default text to display when the field is empty.
    pub placeholder_text: Option<String>,

    /// The color of the placeholder text.
    pub placeholder_color: Color,
}

impl Default for TextField {
    fn default() -> Self {
        Self {
            text: Default::default(),
            cursor_pos: 0,
            cursor_blink_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
            cursor_blink: true,
            active: false,
            selection: None,
            font: Default::default(),
            font_size: 16.0,
            font_color: Color::BLACK,
            placeholder_text: None,
            placeholder_color: Color::GRAY,
        }
    }
}

impl TextField {
    /// Resets the cursor blink timer.
    pub fn reset_cursor_blink(&mut self) {
        self.cursor_blink_timer.reset();
        self.cursor_blink = true;
    }

    /// Sets this text field to be active and resets the cursor blink timer.
    pub fn activate(&mut self) {
        self.active = true;
        self.reset_cursor_blink();
    }

    /// Sets this text field to be inactive.
    pub fn deactivate(&mut self) {
        self.active = false;
    }

    /// Updates the cursor blink state.
    ///
    /// This method returns true if the cursor state has changed.
    pub fn cursor_blink(&mut self, time: &Res<Time>) -> bool {
        if self.cursor_blink_timer.tick(time.delta()).just_finished() {
            self.cursor_blink = !self.cursor_blink;
            return true;
        }
        false
    }

    /// Removes all contents that are currently selected.
    ///
    /// This method will do nothing if there is no selection. Otherwise, it will
    /// remove the selected text and set the cursor position to the start of the
    /// selection. The selection will be cleared after this operation.
    pub fn drain_selection(&mut self) {
        if let Some(selection) = self.selection {
            self.text.drain(selection.start() .. selection.end());
            self.cursor_pos = selection.start();
            self.selection = None;
        }
    }

    /// Inserts a character at the current cursor position as if the user typed
    /// it.
    ///
    /// If there is a selection, it will be replaced by the newly inserted
    /// character.
    pub fn insert_char(&mut self, c: char) {
        self.drain_selection();
        self.text.insert(self.cursor_pos, c);
        self.cursor_pos += 1;
    }

    /// Removes the character before the current cursor position.
    pub fn remove_previous_char(&mut self) {
        if self.cursor_pos > 0 {
            self.text.remove(self.cursor_pos - 1);
            self.cursor_pos -= 1;
        }
    }

    /// Removes the character after the current cursor position.
    pub fn remove_next_char(&mut self) {
        if self.cursor_pos < self.text.len() {
            self.text.remove(self.cursor_pos);
        }
    }

    /// Updates the text sections within the text component based on the current
    /// state of this text field.
    pub fn update_text(&self, text: &mut Text) {
        self.initialize_text_sections(text, false);
        self.clear_sections(text);

        let cursor = if self.cursor_blink { "|" } else { "" };

        if let Some(selection) = self.selection {
            if self.cursor_pos < selection.start() {
                text.sections[0].value = self.text.chars().take(self.cursor_pos).collect();
                text.sections[1].value = cursor.to_owned();
                text.sections[2].value = self
                    .text
                    .chars()
                    .skip(self.cursor_pos)
                    .take(selection.start() - self.cursor_pos)
                    .collect();
                text.sections[3].value = self
                    .text
                    .chars()
                    .skip(selection.start())
                    .take(selection.length())
                    .collect();
                text.sections[6].value = self.text.chars().skip(selection.end()).collect();
            } else if self.cursor_pos < selection.end() {
                text.sections[0].value = self.text.chars().take(selection.start()).collect();
                text.sections[3].value = self
                    .text
                    .chars()
                    .skip(selection.start())
                    .take(self.cursor_pos - selection.start())
                    .collect();
                text.sections[4].value = cursor.to_owned();
                text.sections[5].value = self
                    .text
                    .chars()
                    .skip(self.cursor_pos)
                    .take(selection.end() - self.cursor_pos)
                    .collect();
                text.sections[6].value = self.text.chars().skip(selection.end()).collect();
            } else {
                text.sections[0].value = self.text.chars().take(selection.start()).collect();
                text.sections[3].value = self
                    .text
                    .chars()
                    .skip(selection.start())
                    .take(selection.length())
                    .collect();
                text.sections[6].value = self
                    .text
                    .chars()
                    .skip(selection.end())
                    .take(self.cursor_pos - selection.end())
                    .collect();
                text.sections[7].value = cursor.to_owned();
                text.sections[8].value = self.text.chars().skip(self.cursor_pos).collect();
            }
        } else {
            text.sections[0].value = self.text.chars().take(self.cursor_pos).collect();
            text.sections[1].value = cursor.to_owned();
            text.sections[2].value = self.text.chars().skip(self.cursor_pos).collect();
        }
    }

    /// Initializes the text sections of the given text component.
    ///
    /// If a text component already has 9 sections, this method will do nothing,
    /// unless `force` is set to `true`. Force should be used when changing the
    /// font or font size of the text field.
    pub fn initialize_text_sections(&self, text: &mut Text, force: bool) {
        if !force && text.sections.len() == 9 {
            return;
        }

        let normal_style = TextStyle {
            font: self.font.clone(),
            font_size: self.font_size,
            color: self.font_color,
        };

        let cursor_style = TextStyle {
            font: CURSOR_HANDLE.clone(),
            font_size: self.font_size,
            color: self.font_color,
        };

        let selected_style = TextStyle {
            font: self.font.clone(),
            font_size: self.font_size,
            color: Color::BLUE,
        };

        // Not all sections are used at once, but they are all initialized here
        // for allocation purposes. It speeds up the process of updating the
        // text sections later.
        text.sections = vec![
            // Pre-Selection Pre-cursor
            TextSection::new(String::default(), normal_style.clone()),
            // Pre-Selection Cursor
            TextSection::new(String::default(), cursor_style.clone()),
            // Post-cursor
            TextSection::new(String::default(), normal_style.clone()),
            // Selection Pre-cursor
            TextSection::new(String::default(), selected_style.clone()),
            // Selection Cursor
            TextSection::new(String::default(), cursor_style.clone()),
            // Selection Post-cursor
            TextSection::new(String::default(), selected_style.clone()),
            // Post-Selection Pre-cursor
            TextSection::new(String::default(), normal_style.clone()),
            // Post-Selection Cursor
            TextSection::new(String::default(), cursor_style.clone()),
            // Post-Selection Post-cursor
            TextSection::new(String::default(), normal_style.clone()),
        ];
    }

    /// Clears the text from all sections of the given text component, but
    /// maintains the styles of the sections.
    fn clear_sections(&self, text: &mut Text) {
        for section in text.sections.iter_mut() {
            section.value.clear();
        }
    }
}

/// Represents a region of text that is currently selected.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TextSelection {
    /// The start index of the selection.
    start: usize,

    /// The length of the selection.
    length: usize,
}

impl TextSelection {
    /// Creates a new text selection.
    pub fn new(start: usize, length: usize) -> Self {
        Self { start, length }
    }

    /// Returns the start index of the selection.
    pub fn start(&self) -> usize {
        self.start
    }

    /// Returns the length of the selection.
    pub fn length(&self) -> usize {
        self.length
    }

    /// Returns the end index of the selection.
    pub fn end(&self) -> usize {
        self.start + self.length
    }

    /// Returns whether or not the selection is empty.
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// Modifies the start and end of this text selection to include the given
    /// character index.
    pub fn include_char_at(&mut self, index: usize) {
        if index < self.start {
            self.length += self.start - index;
            self.start = index;
        } else if index >= self.end() {
            self.length = index - self.start + 1;
        }
    }
}

/// This system handles text input for text fields.
pub(crate) fn handle_text_input(
    keyboard_state: Res<ButtonInput<KeyCode>>,
    mut keyboard_input_evs: EventReader<KeyboardInput>,
    mut text_fields: Query<(&mut Text, &mut TextField)>,
) {
    if keyboard_input_evs.is_empty() {
        return;
    }

    for ev in keyboard_input_evs.read() {
        for (mut text, mut field) in text_fields.iter_mut() {
            if !field.active {
                continue;
            }

            if ev.state != ButtonState::Pressed {
                continue;
            }

            let mut dirty = false;

            let ctl_key = keyboard_state.pressed(KeyCode::ControlLeft)
                || keyboard_state.pressed(KeyCode::ControlRight);
            let shift_key = keyboard_state.pressed(KeyCode::ShiftLeft)
                || keyboard_state.pressed(KeyCode::ShiftRight);

            match ev.key_code {
                KeyCode::Backspace => {
                    if field.selection.is_some() {
                        field.drain_selection();
                    } else {
                        field.remove_previous_char();
                    }
                    dirty = true;
                }
                KeyCode::Delete => {
                    if field.selection.is_some() {
                        field.drain_selection();
                    } else {
                        field.remove_next_char();
                    }
                    dirty = true;
                }
                KeyCode::ArrowLeft => {
                    if field.cursor_pos > 0 {
                        field.cursor_pos -= 1;
                        if shift_key {
                            let cursor_pos = field.cursor_pos;
                            if let Some(selection) = &mut field.selection {
                                selection.include_char_at(cursor_pos);
                            } else {
                                field.selection = Some(TextSelection::new(field.cursor_pos, 1));
                            }
                        }
                    }
                    dirty = true;
                }
                KeyCode::ArrowRight => {
                    if field.cursor_pos < field.text.len() {
                        field.cursor_pos += 1;
                        if shift_key {
                            let cursor_pos = field.cursor_pos;
                            if let Some(selection) = &mut field.selection {
                                selection.include_char_at(cursor_pos);
                            } else {
                                field.selection = Some(TextSelection::new(field.cursor_pos - 1, 1));
                            }
                        }
                    }
                    dirty = true;
                }
                KeyCode::Home => {
                    if shift_key {
                        if let Some(selection) = &mut field.selection {
                            selection.include_char_at(0);
                        } else {
                            field.selection = Some(TextSelection::new(0, field.cursor_pos));
                        }
                    }
                    field.cursor_pos = 0;
                    dirty = true;
                }
                KeyCode::End => {
                    if shift_key {
                        let len = field.text.len();
                        if let Some(selection) = &mut field.selection {
                            selection.include_char_at(len);
                        } else {
                            field.selection = Some(TextSelection::new(
                                field.cursor_pos,
                                field.text.len() - field.cursor_pos,
                            ));
                        }
                    }
                    field.cursor_pos = field.text.len();
                    dirty = true;
                }
                KeyCode::Enter => {
                    field.drain_selection();
                    field.insert_char('\n');
                    dirty = true;
                }
                KeyCode::Space => {
                    field.drain_selection();
                    field.insert_char(' ');
                    dirty = true;
                }
                KeyCode::Tab => {
                    field.drain_selection();
                    field.insert_char('\t');
                    dirty = true;
                }
                KeyCode::KeyA => {
                    if ctl_key {
                        field.selection = Some(TextSelection::new(0, field.text.len()));
                        dirty = true;
                    }
                }
                // KeyCode::KeyX => {}
                // KeyCode::KeyC => {}
                // KeyCode::KeyV => {}
                _ => {}
            }

            if let Key::Character(ref s) = ev.logical_key {
                field.drain_selection();
                field.insert_char(s.chars().next().unwrap());
                dirty = true;
            }

            if dirty {
                field.reset_cursor_blink();
                field.update_text(&mut text);
            }
        }
    }
}
