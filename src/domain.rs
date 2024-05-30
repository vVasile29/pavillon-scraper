#[derive(Debug)]
pub struct PavillonDishes {
    pub(crate) dishes: Vec<PavillonDish>,
}

#[derive(Debug)]
pub struct PavillonDish {
    pub name: String,
    pub price: f32,
}

impl PavillonDishes {
    pub fn available_side_dishes(&self) -> Vec<SideDish> {
        vec![]
    }
}

enum SideDish {
    Fries,
    CurlyFries,
    PotatoWedges,
    GarlicPotatoes,
}
