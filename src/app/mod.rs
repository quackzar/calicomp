pub mod events;

use ratatui::widgets::ListState;
use tui_textarea::TextArea;

use crate::sys::{self, data::Repostory, db, recipe::DumbRecipe};

#[derive(Clone, Copy)]
pub enum CurrentScreen {
    Main,
    Editing,
    Exiting,
}

#[derive(Clone, Copy)]
pub enum CurrentlyEditing {
    Name,
    Ingredients,
    Description,
    // TODO:
}

pub struct App {
    pub current_recipe: DumbRecipe, // the currently being edited json value.
    pub current_screen: CurrentScreen, // the current screen the user is looking at, and will later determine what is rendered.
    pub currently_editing: Option<CurrentlyEditing>, // the optional state containing which of the key or value pair the user is editing. It is an option, because when the user is not directly editing a key-value pair, this will be set to `None`.
    pub repo: Repostory,
    pub list_state: ListState,
    pub recipes: Vec<String>,
    pub desc_text: TextArea<'static>,
    pub name_text: TextArea<'static>,
}

impl App {
    pub fn new() -> App {
        let mut repo = Repostory::default();
        repo.recipes.insert("Daiquri".to_string(), db::new_daiq());
        repo.recipes.insert("Baiquri".to_string(), db::new_daiq());
        repo.recipes.insert("Caiquri".to_string(), db::new_daiq());
        repo.recipes.insert("aiquri".to_string(), db::new_daiq());

        let recipes = Vec::from_iter(repo.recipes.keys().cloned());
        App {
            repo,
            recipes,
            current_recipe: sys::db::new_daiq().dumb(),
            current_screen: CurrentScreen::Main,
            currently_editing: None,
            desc_text: TextArea::default(),
            name_text: TextArea::default(),
            list_state: ListState::default(),
        }
    }

    pub fn save_current_recipe(&mut self) -> Option<()> {
        let recipe = self.current_recipe.clone();
        let recipe = self.repo.enrich(recipe)?;

        self.repo.recipes.insert(recipe.name.clone(), recipe);

        self.currently_editing = None;
        Some(())
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
