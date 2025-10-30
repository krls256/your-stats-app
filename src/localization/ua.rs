use super::LocalizationTrait;

pub struct UaLocalization;

impl LocalizationTrait for UaLocalization {
    fn tab_metrics(&self) -> &'static str {
        "Метрики"
    }

    fn tab_add_data(&self) -> &'static str {
        "Додати дані"
    }

    fn tab_correlations(&self) -> &'static str {
        "Кореляції"
    }

    fn app_title(&self) -> &'static str {
        "MVP Grafana для власних даних"
    }

    fn metrics_title(&self) -> &'static str {
        "Колекції метрик"
    }

    fn create_metric(&self) -> &'static str {
        "+ Створити метрику"
    }

    fn new_metric(&self) -> &'static str {
        "Нова метрика"
    }

    fn name(&self) -> &'static str {
        "Назва"
    }

    fn metric_type(&self) -> &'static str {
        "Тип"
    }

    fn description(&self) -> &'static str {
        "Опис"
    }

    fn cancel(&self) -> &'static str {
        "Скасувати"
    }

    fn create_with_enter(&self) -> &'static str {
        "Створити (⏎)"
    }

    fn created_at(&self) -> &'static str {
        "Створено:"
    }

    fn delete_metric(&self) -> &'static str {
        "Видалити метрику"
    }

    fn delete_metric_confirm_title(&self) -> &'static str {
        "Видалити метрику?"
    }

    fn delete_metric_confirm_message(&self) -> &'static str {
        "Ви впевнені, що хочете видалити цю метрику? Всі дані, пов'язані з нею, також будуть видалені. Цю дію неможливо скасувати."
    }

    fn delete(&self) -> &'static str {
        "Видалити"
    }

    fn add_data_title(&self) -> &'static str {
        "Додати дані"
    }

    fn metric(&self) -> &'static str {
        "Метрика"
    }

    fn select_metric(&self) -> &'static str {
        "-- Оберіть --"
    }

    fn value(&self) -> &'static str {
        "Значення"
    }

    fn add(&self) -> &'static str {
        "Додати"
    }

    fn add_with_enter(&self) -> &'static str {
        "Додати (⏎)"
    }

    fn data_added_success(&self) -> &'static str {
        "✅ Дані успішно додано!"
    }

    fn data_add_error(&self) -> &'static str {
        "Помилка додавання даних:"
    }

    fn select_metric_first(&self) -> &'static str {
        "Оберіть метрику спочатку"
    }

    fn records(&self) -> &'static str {
        "Записи"
    }

    fn previous(&self) -> &'static str {
        "Назад"
    }

    fn next(&self) -> &'static str {
        "Вперед"
    }
    
    fn page_formatted(&self, current_page: usize, total_pages: usize) -> String {
        format!("Сторінка {} з {}", current_page, total_pages)
    }

    fn correlations_title(&self) -> &'static str {
        "Кореляції між метриками"
    }

    fn calculate_correlations(&self) -> &'static str {
        "🔄 Обчислити кореляції"
    }

    fn click_to_calculate(&self) -> &'static str {
        "📊 Натисніть кнопку вище, щоб обчислити кореляції"
    }

    fn need_numeric_metrics(&self) -> &'static str {
        "Потрібно мінімум 2 числові метрики з даними"
    }

    fn boolean_metrics(&self) -> &'static str {
        "🔘 Булеві метрики"
    }

    fn numeric_metrics(&self) -> &'static str {
        "🔢 Числові метрики (Integer / Float)"
    }

    fn strong_correlation(&self) -> &'static str {
        "Сильна кореляція"
    }

    fn medium_correlation(&self) -> &'static str {
        "Середня кореляція"
    }

    fn weak_correlation(&self) -> &'static str {
        "Слабка кореляція"
    }
    
    fn based_on_points_formatted(&self, count: usize) -> String {
        format!("На основі {} точок даних", count)
    }
}

pub static UA: UaLocalization = UaLocalization;
