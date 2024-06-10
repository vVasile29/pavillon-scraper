use chrono::NaiveDate;
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

    pub fn get_date(&self) -> Option<NaiveDate> {
        let file_name = self.url.path_segments().unwrap().last()?;
        let numbers_only: String = file_name.chars().filter(char::is_ascii_digit).collect();
        NaiveDate::parse_from_str(&numbers_only, "%d%m%Y").ok()
    }
}

// strings need to be static slices to be able to be used in a const context
#[derive(Debug)]
pub struct SideDish {
    pub colloquial_name: &'static str,
    keywords: &'static [&'static str],
    pub emojis: Option<&'static str>,
}

impl SideDish {
    const fn new(
        colloquial_name: &'static str,
        keywords: &'static [&'static str],
        emojis: Option<&'static str>,
    ) -> Self {
        SideDish {
            colloquial_name,
            keywords,
            emojis,
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
    SideDish::new("Pommes Frites", &["pommes"], Some("🍟")),
    SideDish::new("Twisterkartoffeln", &["twisterkartoffeln"], Some("🌪")),
    SideDish::new("Kartoffelwedges", &["wedges"], Some("🥔📐")),
    SideDish::new("Knoblauchkartoffeln", &["knoblauchkartoffeln"], Some("🧄")),
];
