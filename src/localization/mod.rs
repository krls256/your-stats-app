use strum_macros::{EnumIter, FromRepr};
use strum::{IntoEnumIterator};

pub mod ua;
pub mod en;

pub trait LocalizationTrait: Send + Sync {
    fn tab_metrics(&self) -> &'static str;
    fn tab_add_data(&self) -> &'static str;
    fn tab_correlations(&self) -> &'static str;

    fn app_title(&self) -> &'static str;

    fn metrics_title(&self) -> &'static str;
    fn create_metric(&self) -> &'static str;
    fn new_metric(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn metric_type(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn cancel(&self) -> &'static str;
    fn create_with_enter(&self) -> &'static str;
    fn created_at(&self) -> &'static str;
    fn delete_metric(&self) -> &'static str;
    fn delete_metric_confirm_title(&self) -> &'static str;
    fn delete_metric_confirm_message(&self) -> &'static str;
    fn delete(&self) -> &'static str;

    fn add_data_title(&self) -> &'static str;
    fn metric(&self) -> &'static str;
    fn select_metric(&self) -> &'static str;
    fn value(&self) -> &'static str;
    fn add_with_enter(&self) -> &'static str;
    fn data_added_success(&self) -> &'static str;
    fn data_add_error(&self) -> &'static str;
    fn select_metric_first(&self) -> &'static str;
    fn records(&self) -> &'static str;
    fn previous(&self) -> &'static str;
    fn next(&self) -> &'static str;
    
    fn page_formatted(&self, current_page: usize, total_pages: usize) -> String;

    fn correlations_title(&self) -> &'static str;
    fn calculate_correlations(&self) -> &'static str;
    fn click_to_calculate(&self) -> &'static str;
    fn need_numeric_metrics(&self) -> &'static str;
    fn boolean_metrics(&self) -> &'static str;
    fn numeric_metrics(&self) -> &'static str;
    fn strong_correlation(&self) -> &'static str;
    fn medium_correlation(&self) -> &'static str;
    fn weak_correlation(&self) -> &'static str;
    
    fn based_on_points_formatted(&self, count: usize) -> String;
}

#[derive(Clone, Copy, PartialEq, Eq, EnumIter, FromRepr)]
pub enum Language {
    Ua,
    En,
}

impl Language {
    pub fn all() -> impl Iterator<Item=Self> {
        Language::iter().rev()
    }
    pub fn as_str(&self) -> &'static str {
        match self {
            Language::Ua => "UA",
            Language::En => "EN"
        }
    }
}


pub fn get_localization(lang: Language) -> &'static dyn LocalizationTrait {
    match lang {
        Language::Ua => &ua::UA,
        Language::En => &en::EN,
    }
}