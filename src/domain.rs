#[derive(Debug)]
struct PavillonDishes {
    dishes: Vec<PavillonDish>,
}

#[derive(Debug)]
struct PavillonDish {
    name: String,
    price: f32,
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
