use dioxus::prelude::*;
use crate::domain::{metric::Metric, data_point::DataPoint, offset};
use crate::application::Service;
use crate::localization::{Language, get_localization};

#[component]
pub fn AddDataTab(
    metrics: Signal<Vec<Metric>>,
    mut selected_metric: Signal<Option<i64>>,
    mut error_message: Signal<Option<String>>,
    language: Language,
    srv: Signal<&'static Service>,
) -> Element {
    let localization = get_localization(language);
    let mut data_value = use_signal(String::new);
    let mut success_message = use_signal(|| None::<String>);
    let mut recent_points = use_signal(Vec::<DataPoint>::new);
    let mut current_page = use_signal(|| 1);
    let mut total_count = use_signal(|| 0);

    const PAGE_SIZE: usize = 10;

    let mut last_metric_id = use_signal(|| None::<i64>);

    use_effect(use_reactive!(|selected_metric, current_page| {
        let metric_id = selected_metric();
        let page = current_page();

        let metric_changed = last_metric_id() != metric_id;
        if metric_changed {
            last_metric_id.set(metric_id);
        }

        if let Some(metric_id) = metric_id {
            if let Ok(count) = srv.read().get_data_points_count(metric_id) {
                total_count.set(count);
            }

            if let Ok(points) = srv.read().get_data_points_paginated(metric_id, PAGE_SIZE, offset(page, PAGE_SIZE)) {
                recent_points.set(points);
            }
        } else {
            recent_points.set(Vec::new());
            total_count.set(0);
        }
    }));

    let mut add_data_action = move || {

        if let Some(metric_id) = selected_metric() {
            match srv.read().add_data_point(metric_id, &data_value()) {
                Ok(_) => {
                    data_value.set(String::new());
                    success_message.set(Some(localization.data_added_success().to_string()));
                    error_message.set(None);
                    current_page.set(1);
                }
                Err(e) => {
                    error_message.set(Some(format!("{} {}", localization.data_add_error(), e)));
                    success_message.set(None);
                }
            }
        } else {
            error_message.set(Some(localization.select_metric_first().to_string()));
        }
    };

    let add_data = move |_| add_data_action();

    let mut delete_point_action = move |point_id: i64| {
        if srv.read().delete_data_point(point_id).is_ok() {
            if let Some(metric_id) = selected_metric() {
                if let Ok(count) = srv.read().get_data_points_count(metric_id) {
                    total_count.set(count);
                }

                if let Ok(points) = srv.read().get_data_points_paginated(metric_id, PAGE_SIZE, offset(current_page(), PAGE_SIZE)) {
                    recent_points.set(points);
                }

                if recent_points().is_empty() && current_page() > 1 {
                    let new_page = current_page() - 1;
                    current_page.set(new_page);

                    if let Ok(points) = srv.read().get_data_points_paginated(metric_id, PAGE_SIZE, offset(new_page, PAGE_SIZE)) {
                        recent_points.set(points);
                    }
                }
            }
        }
    };

    rsx! {
        div { class: "space-y-4",
            h2 { class: "text-2xl font-bold text-gray-900", "{localization.add_data_title()}" }
            div { class: "bg-white rounded-lg shadow-md p-6 mb-4",
                div { class: "mb-4",
                    label { class: "block text-sm font-medium text-gray-700 mb-1", "{localization.metric()}" }
                    select { 
                        class: "w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-900 focus:border-transparent appearance-none bg-white",
                        value: "{selected_metric().unwrap_or(0)}",
                        onchange: move |e| {
                            if let Ok(id) = e.value().parse::<i64>() {
                                if id > 0 {
                                    selected_metric.set(Some(id));
                                    current_page.set(1);
                                } else {
                                    selected_metric.set(None);
                                }
                            }
                        },
                        option { value: "0", "{localization.select_metric()}" }
                        for metric in metrics() { option { value: "{metric.id}", "{metric.name} ({metric.metric_type:?})" } }
                    }
                }

                if selected_metric().is_some() {
                    div { class: "mb-4",
                        label { class: "block text-sm font-medium text-gray-700 mb-1", "{localization.value()}" }
                        input { 
                            class: "w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-900 focus:border-transparent",
                            value: "{data_value}", 
                            oninput: move |e| data_value.set(e.value()),
                            onkeydown: move |e| {
                                if e.key() == Key::Enter && !data_value().is_empty() {
                                    add_data_action();
                                }
                            }
                        }
                    }

                    div { class: "flex justify-end",
                        button { 
                            class: "px-4 py-2 rounded-lg bg-blue-900 text-white font-semibold hover:bg-blue-800 transition-colors disabled:opacity-50 disabled:cursor-not-allowed", 
                            onclick: add_data, 
                            disabled: data_value().is_empty(), 
                            "{localization.add_with_enter()}" 
                        }
                    }
                }
            }

            if let Some(msg) = success_message() { 
                div { class: "bg-green-50 border border-green-200 text-green-700 px-4 py-3 rounded-lg", "{msg}" } 
            }

            if !recent_points().is_empty() || total_count() > 0 {
                div { class: "bg-white rounded-lg shadow-md p-5",
                    div { class: "flex justify-between items-center mb-3",
                        h3 { class: "text-lg font-semibold text-gray-900", 
                            "{localization.records()} ({total_count()})"
                        }
                    }
                    if !recent_points().is_empty() {
                        for point in recent_points() {
                            div { key: "{point.id}", class: "flex items-center gap-4 py-2 border-b border-gray-100 last:border-0",
                                span { class: "font-medium text-gray-900 min-w-[160px]", "{point.value}" }
                                span { class: "text-sm text-gray-500 flex-1", "{point.readable_timestamp()}" }
                                button {
                                    class: "px-2 py-1 text-red-600 hover:bg-red-50 rounded transition-colors text-sm",                            
                                    onclick: move |_| delete_point_action(point.id),
                                    "🗑️"
                                }
                            }
                        }
                    }

                    if total_count() > PAGE_SIZE {
                        crate::presentation::pagination::Pagination {
                            current_page: current_page,
                            total_count: total_count(),
                            page_size: PAGE_SIZE,
                            on_page_change: move |new_page: usize| {
                                if let Some(metric_id) = selected_metric() {
                                    let offset = (new_page - 1) * PAGE_SIZE;

                                    if let Ok(points) = srv.read().get_data_points_paginated(metric_id, PAGE_SIZE, offset) {
                                        recent_points.set(points);
                                    }
                                }
                            },
                            language: language
                        }
                    }
                }
            }
        }
    }
}
