pub mod data;
pub mod events;

use tui_textarea::TextArea;

use crate::{
    app::data::Repostory,
    sys::recipe::{Product, Recipe},
};

pub enum CurrentScreen {
    Main,
    Editing,
    Exiting,
}

pub enum CurrentlyEditing {
    Name,
    Ingredients,
    Description,
    // TODO:
}

pub struct App {
    pub current_recipe: Recipe, // the currently being edited json value.
    pub current_screen: CurrentScreen, // the current screen the user is looking at, and will later determine what is rendered.
    pub currently_editing: Option<CurrentlyEditing>, // the optional state containing which of the key or value pair the user is editing. It is an option, because when the user is not directly editing a key-value pair, this will be set to `None`.
    pub repo: Repostory,
    pub desc_text: TextArea<'static>,
    pub name_text: TextArea<'static>,
}

impl App {
    pub fn new() -> App {
        App {
            current_recipe: Recipe::new("Unnamed Recipe".to_string()),
            repo: Repostory::default(),
            current_screen: CurrentScreen::Main,
            currently_editing: None,
            desc_text: TextArea::default(),
            name_text: TextArea::default(),
        }
    }

    pub fn save_current_recipe(&mut self) {
        let recipe  = Recipe::builder()
            .name(self.name_text.lines().concat())
            .description(self.desc_text.lines().concat())
            .build();

        self.repo.recipes.insert( recipe.name.clone(), recipe);

        self.currently_editing = None;
    }

    pub fn toggle_editing(&mut self) {
        if let Some(edit_mode) = &self.currently_editing {
            match edit_mode {
                CurrentlyEditing::Name => {
                    self.currently_editing = Some(CurrentlyEditing::Description)
                }
                CurrentlyEditing::Description => {
                    self.currently_editing = Some(CurrentlyEditing::Name)
                }
                _ => todo!(),
            };
        } else {
            self.currently_editing = Some(CurrentlyEditing::Name);
        }
    }

    pub fn print_toml(&self) -> Result<(), toml::ser::Error> {
        let output = toml::to_string(&self.repo)?;
        println!("{}", output);
        Ok(())
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
