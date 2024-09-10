
use measurements::Volume;

use crate::recipe::{Datasheet, Product, Recipe, RecipeBuilder};


pub fn generate() -> Vec<Recipe> {
    let rum = Product::builder()
        .datasheet(Datasheet::builder().abv(40.0).build())
        .name("Rum".to_string()).build();

    let simple = Product::builder()
        .datasheet(Datasheet::builder().brix(50.0).build())
        .name("Simple Sirup".to_string()).build();

    let lime = Product::builder()
        .datasheet(Datasheet::builder().acidity(6.0).brix(1.7).build())
        .name("Lime Juice".to_string()).build();

    let daiquri = Recipe::builder()
        .ingredients(vec![
            (Volume::from_milliliters(60.0), rum),
            (Volume::from_milliliters(20.0), simple),
            (Volume::from_milliliters(20.0), lime),
        ])
        .name("Daiquiri".to_string())
        .dilution(20.0)
        .build();

    vec![
        daiquri
    ]
}
