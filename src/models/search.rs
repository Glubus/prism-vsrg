#[derive(Debug, Clone, PartialEq)]
pub struct MenuSearchFilters {
    pub query: String,
    pub min_rating: Option<f64>,
    pub max_duration_seconds: Option<f64>,
}

impl Default for MenuSearchFilters {
    fn default() -> Self {
        Self {
            query: String::new(),
            min_rating: None,
            max_duration_seconds: None,
        }
    }
}

impl MenuSearchFilters {
    pub fn is_active(&self) -> bool {
        !self.query.trim().is_empty()
            || self.min_rating.is_some()
            || self.max_duration_seconds.is_some()
    }
}
