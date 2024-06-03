use reqwest::Url;
use std::path::PathBuf;

#[derive(Debug)]
pub struct PavillonDishes {
    pub url: Url,
    pub path: PathBuf,
    pub dishes: Vec<PavillonDish>,
}

#[derive(Debug)]
pub struct PavillonDish {
    pub name: String,
    pub price: f32,
}

impl PavillonDish {
    pub fn available_side_dishes(&self) -> Vec<SideDish> {
        vec![]
    }
}

impl PavillonDishes {
    pub fn available_side_dishes(&self) -> Vec<SideDish> {
        vec![]
    }
}

pub enum SideDish {
    Fries,
    CurlyFries,
    PotatoWedges,
    GarlicPotatoes,
}
