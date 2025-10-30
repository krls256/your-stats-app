mod domain;
mod repository;
mod application;
mod presentation;
mod localization;

use domain::{correlation::Correlation, data_point::DataPointDTO, metric::Metric, MetricType};
use dioxus::prelude::*;
use std::sync::{Arc, Mutex};

fn main() {
    application::init_db("metrics.db");
    dioxus::launch(app);
}

#[component]
fn app() -> Element {
    let mut metrics = use_signal(|| Vec::<Metric>::new());
    let mut correlations = use_signal(|| Vec::<Correlation>::new());
    let mut selected_metric = use_signal(|| None::<i64>);
    let mut error_message = use_signal(|| None::<String>);
    let mut active_tab = use_signal(|| presentation::navigation::Tab::Metrics);
    let mut language = use_signal(|| localization::Language::Ua);

    use_effect(move || {
        if let Ok(m) = application::list_metrics() { metrics.set(m); }
    });

    let tabs = vec![
        presentation::navigation::Tab::Metrics,
        presentation::navigation::Tab::AddData,
        presentation::navigation::Tab::Correlations,
    ];

    rsx! {
        script { src: "https://cdn.tailwindcss.com" }
        div { class: "min-h-screen bg-gray-50",
            div { class: "max-w-6xl mx-auto px-8 py-8",
                div { class: "bg-white rounded-lg shadow-md p-6 mb-6",
                    div { class: "flex justify-between items-center mb-4",
                        h1 { class: "text-3xl font-bold text-gray-900", "{localization::get_localization(language()).app_title()}" }
                        div { class: "flex gap-2",
                            button {
                                class: if language() == localization::Language::Ua {
                                    "px-3 py-1 rounded bg-blue-900 text-white text-sm"
                                } else {
                                    "px-3 py-1 rounded bg-gray-200 text-gray-700 text-sm hover:bg-gray-300"
                                },
                                onclick: move |_| language.set(localization::Language::Ua),
                                "UA"
                            }
                            button {
                                class: if language() == localization::Language::En {
                                    "px-3 py-1 rounded bg-blue-900 text-white text-sm"
                                } else {
                                    "px-3 py-1 rounded bg-gray-200 text-gray-700 text-sm hover:bg-gray-300"
                                },
                                onclick: move |_| language.set(localization::Language::En),
                                "EN"
                            }
                        }
                    }
                    presentation::navigation::Navigation {
                        tabs: tabs.clone(),
                        active_tab: active_tab,
                        language: language()
                    }
                }

                if let Some(err) = error_message() { 
                    div { class: "bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded-lg mb-6", "{err}" } 
                }

                match active_tab() {
                    presentation::navigation::Tab::Metrics => rsx! { presentation::metrics_tab::MetricsTab { metrics: metrics, language: language() } },
                    presentation::navigation::Tab::AddData => rsx! { presentation::add_data_tab::AddDataTab { metrics: metrics, selected_metric: selected_metric, error_message: error_message, language: language() } },
                    presentation::navigation::Tab::Correlations => rsx! { presentation::correlations_tab::CorrelationsTab { correlations: correlations, language: language() } },
                }
            }
        }
    }
}
