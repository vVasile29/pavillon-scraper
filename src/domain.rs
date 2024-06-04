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
        SideDish::matching_side_dishes(&self.name)
    }
}

impl PavillonDishes {
    pub fn available_side_dishes(&self) -> Vec<SideDish> {
        self.dishes
            .iter()
            .flat_map(|dish| dish.available_side_dishes())
            .collect()
    }
}

// strings need to be static slices to be able to be used in a const context
#[derive(Debug)]
pub struct SideDish {
    pub colloquial_name: &'static str,
    keywords: &'static [&'static str],
    pub emoji: Option<char>,
}

impl SideDish {
    const fn new(
        colloquial_name: &'static str,
        keywords: &'static [&'static str],
        emoji: Option<char>,
    ) -> Self {
        SideDish {
            colloquial_name,
            keywords,
            emoji,
        }
    }
    fn matching_side_dishes<S: AsRef<str>>(string: S) -> Vec<SideDish> {
        SIDE_DISHES
            .into_iter()
            .filter(|dish| dish.matches(string.as_ref()))
            .collect()
    }

    fn matches<S: AsRef<str>>(&self, string: S) -> bool {
        self.keywords
            .iter()
            .any(|key| string.as_ref().to_lowercase().contains(key))
    }
}

const SIDE_DISHES: [SideDish; 4] = [
    SideDish::new("Pommes Frites", &["pommes"], Some('🍟')),
    SideDish::new("Spiralpommes", &[], None),
    SideDish::new("Kartoffelecken", &[], None),
    SideDish::new("Knoblauchkartoffeln", &["knoblauchkartoffeln"], Some('🧄')),
];
