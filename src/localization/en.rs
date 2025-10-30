use super::LocalizationTrait;

pub struct EnLocalization;

impl LocalizationTrait for EnLocalization {
    fn tab_metrics(&self) -> &'static str {
        "Metrics"
    }

    fn tab_add_data(&self) -> &'static str {
        "Add Data"
    }

    fn tab_correlations(&self) -> &'static str {
        "Correlations"
    }

    fn app_title(&self) -> &'static str {
        "MVP Grafana for Your Data"
    }

    fn metrics_title(&self) -> &'static str {
        "Metric Collections"
    }

    fn create_metric(&self) -> &'static str {
        "+ Create Metric"
    }

    fn new_metric(&self) -> &'static str {
        "New Metric"
    }

    fn name(&self) -> &'static str {
        "Name"
    }

    fn metric_type(&self) -> &'static str {
        "Type"
    }

    fn description(&self) -> &'static str {
        "Description"
    }

    fn cancel(&self) -> &'static str {
        "Cancel"
    }

    fn create_with_enter(&self) -> &'static str {
        "Create (⏎)"
    }

    fn created_at(&self) -> &'static str {
        "Created:"
    }

    fn delete_metric(&self) -> &'static str {
        "Delete Metric"
    }

    fn delete_metric_confirm_title(&self) -> &'static str {
        "Delete Metric?"
    }

    fn delete_metric_confirm_message(&self) -> &'static str {
        "Are you sure you want to delete this metric? All associated data will also be deleted. This action cannot be undone."
    }

    fn delete(&self) -> &'static str {
        "Delete"
    }

    fn add_data_title(&self) -> &'static str {
        "Add Data"
    }

    fn metric(&self) -> &'static str {
        "Metric"
    }

    fn select_metric(&self) -> &'static str {
        "-- Select --"
    }

    fn value(&self) -> &'static str {
        "Value"
    }

    fn add(&self) -> &'static str {
        "Add"
    }

    fn add_with_enter(&self) -> &'static str {
        "Add (⏎)"
    }

    fn data_added_success(&self) -> &'static str {
        "✅ Data added successfully!"
    }

    fn data_add_error(&self) -> &'static str {
        "Error adding data:"
    }

    fn select_metric_first(&self) -> &'static str {
        "Please select a metric first"
    }

    fn records(&self) -> &'static str {
        "Records"
    }

    fn previous(&self) -> &'static str {
        "Previous"
    }

    fn next(&self) -> &'static str {
        "Next"
    }
    
    fn page_formatted(&self, current_page: usize, total_pages: usize) -> String {
        format!("Page {} of {}", current_page, total_pages)
    }

    fn correlations_title(&self) -> &'static str {
        "Correlations between Metrics"
    }

    fn calculate_correlations(&self) -> &'static str {
        "🔄 Calculate Correlations"
    }

    fn click_to_calculate(&self) -> &'static str {
        "📊 Click the button above to calculate correlations"
    }

    fn need_numeric_metrics(&self) -> &'static str {
        "At least 2 numeric metrics with data are required"
    }

    fn boolean_metrics(&self) -> &'static str {
        "🔘 Boolean Metrics"
    }

    fn numeric_metrics(&self) -> &'static str {
        "🔢 Numeric Metrics (Integer / Float)"
    }

    fn strong_correlation(&self) -> &'static str {
        "Strong Correlation"
    }

    fn medium_correlation(&self) -> &'static str {
        "Medium Correlation"
    }

    fn weak_correlation(&self) -> &'static str {
        "Weak Correlation"
    }
    
    fn based_on_points_formatted(&self, count: usize) -> String {
        format!("Based on {} data points", count)
    }
}

pub static EN: EnLocalization = EnLocalization;

