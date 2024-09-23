use std::collections::BTreeMap;

use measurements::Volume;
use serde::{
    ser::{SerializeMap, SerializeStruct},
    Deserialize, Serialize,
};

use crate::sys::{
    glass::Glassware,
    recipe::{Product, Recipe},
};

#[derive(Serialize, Deserialize, Default)]
pub struct Reposotory {
    pub recipes: BTreeMap<String, Recipe>,
    pub ingredients: BTreeMap<String, Product>,
}

impl Reposotory {
    pub fn enrich(&self, recipe: DumbRecipe) -> Option<Recipe> {
        let DumbRecipe {
            name,
            short_desc,
            description,
            ingredients,
            dilution,
            glassware,
        } = recipe;
        let ingredients = ingredients
            .into_iter()
            .map(|(v, i)| {
                let i = self.ingredients.get(&i)?.clone();
                Some((Volume::from_milliliters(v), i))
            })
            .collect::<Option<_>>()?;
        Some(Recipe {
            name,
            short_desc,
            description,
            ingredients,
            dilution,
            glassware,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DumbRecipe {
    pub name: String,

    #[serde(default)]
    pub short_desc: Option<String>,

    #[serde(default)]
    pub description: Option<String>,

    #[serde(default)]
    pub ingredients: Vec<(f64, String)>,

    #[serde(default)]
    pub dilution: f64,

    #[serde(default)]
    pub glassware: Option<Glassware>,
}

#[derive(Clone, Debug)]
pub struct IngredientLine {
    pub ingredient: String,
    pub amount: f64,
    pub unit: String,
}
