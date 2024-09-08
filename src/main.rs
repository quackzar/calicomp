pub mod ui;

use bon::builder;
use chrono::NaiveDate;
use measurements::Volume;
use serde::{Deserialize, Serialize};

#[builder]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Ingredient {
    name: String,
    #[serde(default)]
    description: Option<String>,
    // per 100g or 100 ml
    #[serde(default)]
    abv: f64,
    #[serde(default)]
    brix: f64,
    #[serde(default)]
    fat: f64,
    #[serde(default)]
    density: f64,
    #[serde(default)]
    acidity: f64,
}

pub fn abv_to_abw(abv: f64) -> f64 {
    // https://alcohol.stackexchange.com/a/6499
    0.1893*abv*abv + 0.7918*abv + 0.0002
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnitKind {
    Volume,
    Quanity,
    Mass
}


#[builder]
#[derive(Debug, Clone, PartialEq)]
pub struct Item {
    kind: Ingredient,
    full_size: Option<f64>,
    current_size: Option<f64>,
    measurement: UnitKind,
    label: Option<String>,
    expiry: Option<NaiveDate>,
    opened: Option<NaiveDate>,
}

#[derive(Clone, PartialEq)]
pub enum Quantity {
    Mass(measurements::Mass),
    Volume(measurements::Volume),
    Countable(u32),
}

#[builder]
#[derive(Clone, PartialEq)]
pub struct Recipe {
    name: String,
    description: String,
    steps: Vec<String>,
    ingredients: Vec<(measurements::Volume, Ingredient)>,
    /// percentage
    dilution: f64,
}


#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Preperation {
    Stirred,
    Shaken,
    Blended,
    BuildInGlass,
    Other
}

impl Recipe {
    pub fn calc_volume(&self) -> Volume {
        // Note; There is some volume change when mixing different abv
        let milis = self.ingredients.iter().map(|(volume, _)| {
            volume.as_milliliters()
        }).sum();
        Volume::from_milliliters(milis) * ((self.dilution / 100.0) + 1.0)
    }

    pub fn calc_abv(&self) -> f64 {
        // https://jeffreymorgenthaler.com/cocktail-abv-calculator/
        let milis = self.ingredients.iter()
            .map(|(volume, ingredient)| {
                volume.as_milliliters() * ingredient.abv
            }).sum();
        let alcohol = Volume::from_milliliters(milis);
        alcohol / self.calc_volume()
    }

    pub fn calc_brix(&self) -> f64 {
        let milis = self.ingredients.iter()
            .map(|(volume, ingredient)| {
                volume.as_milliliters() * ingredient.brix
            }).sum();
        let sugar_in_solution = Volume::from_milliliters(milis);
        sugar_in_solution / self.calc_volume()
    }
}

fn main() {
    ui::main().unwrap()
}
