use measurements::Volume;

use crate::sys::recipe::{Datasheet, Product, Recipe};

pub fn generate() -> Vec<Recipe> {
    vec![new_daiq()]
}

pub fn new_daiq() -> Recipe {
    let rum = Product::builder()
        .datasheet(Datasheet::builder().abv(40.0).build())
        .name("Rum".to_string())
        .build();

    let simple = Product::builder()
        .datasheet(Datasheet::builder().brix(50.0).build())
        .name("Simple Sirup".to_string())
        .build();

    let lime = Product::builder()
        .datasheet(Datasheet::builder().acidity(6.0).brix(1.7).build())
        .name("Lime Juice".to_string())
        .build();

    Recipe::builder()
        .ingredients(vec![
            (Volume::from_milliliters(60.0), rum),
            (Volume::from_milliliters(20.0), simple),
            (Volume::from_milliliters(20.0), lime),
        ])
        .name("Daiquiri".to_string())
        .dilution(20.0)
        .glassware(super::glass::Glassware::Martini)
        .short_desc("Happy Hour / Summer drink".to_string())
        .description(
            "The daiquiri (/ˈdaɪkəri, ˈdæk-/; Spanish: daiquirí [dajkiˈɾi])\
            is a cocktail whose main ingredients are rum, citrus juice\
            (typically lime juice), and sugar or other sweetener."
                .to_string(),
        )
        .build()
}
