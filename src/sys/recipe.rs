use bon::builder;
use chrono::NaiveDate;
use measurements::Volume;
use serde::{Deserialize, Serialize};

use crate::sys::glass::Glassware;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Ingredient {
    Product(Product),
    Generic {
        name: String,
        description: Option<String>,
        datasheet: Datasheet,
    },
}

impl Ingredient {
    pub fn datasheet(&self) -> &Datasheet {
        match self {
            Ingredient::Product(p) => &p.datasheet,
            Ingredient::Generic {
                name: _,
                description: _,
                datasheet,
            } => datasheet,
        }
    }
}

#[builder]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Product {
    pub name: String,
    #[serde(default)]
    pub brand: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub datasheet: Datasheet,
}

#[builder]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Datasheet {
    #[builder(default)]
    #[serde(default)]
    abv: f64,
    #[builder(default)]
    #[serde(default)]
    brix: f64,
    #[builder(default)]
    #[serde(default)]
    fat: f64,
    #[builder(default)]
    #[serde(default)]
    density: f64,
    #[builder(default)]
    #[serde(default)]
    acidity: f64,
}

pub fn abv_to_abw(abv: f64) -> f64 {
    // https://alcohol.stackexchange.com/a/6499
    0.1893 * abv * abv + 0.7918 * abv + 0.0002
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnitKind {
    Volume,
    Quanity,
    Mass,
}

#[builder]
#[derive(Debug, Clone, PartialEq)]
pub struct Item {
    kind: Product,
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
#[derive(Debug, Clone)]
pub struct Recipe {
    pub name: String,
    pub short_desc: Option<String>,
    pub description: Option<String>,
    #[builder(default)]
    steps: Vec<String>,
    pub ingredients: Vec<(measurements::Volume, Product)>,
    /// percentage
    pub dilution: f64,
    pub glassware: Option<Glassware>,
}

#[derive(Clone, PartialEq, Eq)]
pub enum Preperation {
    Stirred,
    Shaken,
    Blended,
    BuildInGlass,
    Other(String),
}

impl Preperation {
    pub fn dilution(&self) -> f64 {
        match self {
            Preperation::Stirred => 20.0,
            Preperation::Shaken => 20.0,
            Preperation::Blended => f64::NAN,
            Preperation::BuildInGlass => 0.0,
            Preperation::Other(_) => f64::NAN,
        }
    }
}

impl Recipe {
    pub fn calc_volume(&self) -> Volume {
        // Note; There is some volume change when mixing different abv
        let milis = self
            .ingredients
            .iter()
            .map(|(volume, _)| volume.as_milliliters())
            .sum();
        Volume::from_milliliters(milis) * ((self.dilution / 100.0) + 1.0)
    }

    pub fn calc_abv(&self) -> f64 {
        // https://jeffreymorgenthaler.com/cocktail-abv-calculator/
        let milis = self
            .ingredients
            .iter()
            .map(|(volume, ingredient)| volume.as_milliliters() * ingredient.datasheet.abv)
            .sum();
        let alcohol = Volume::from_milliliters(milis);
        alcohol / self.calc_volume()
    }

    pub fn calc_brix(&self) -> f64 {
        let milis = self
            .ingredients
            .iter()
            .map(|(volume, ingredient)| volume.as_milliliters() * ingredient.datasheet.brix)
            .sum();
        let sugar_in_solution = Volume::from_milliliters(milis);
        sugar_in_solution / self.calc_volume()
    }
}
