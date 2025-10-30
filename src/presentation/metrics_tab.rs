use dioxus::prelude::*;
use crate::domain::metric::Metric;
use crate::application as app;
use crate::localization::{get_localization, Language};

#[component]
pub fn MetricsTab(
    mut metrics: Signal<Vec<Metric>>,
    language: Language,
) -> Element {
    let localization = get_localization(language);

    let mut show_form = use_signal(|| false);
    let mut metric_to_delete = use_signal(|| None::<i64>);
    let refresh_trigger = use_signal(|| ());

    use_effect(move || {
        refresh_trigger();
        if let Ok(m) = app::list_metrics() { metrics.set(m); }
    });

    let mut delete_metric_action = move |metric_id: i64| {
        if app::delete_metric(metric_id).is_ok() {
            metric_to_delete.set(None);

            if let Ok(m) = app::list_metrics() { metrics.set(m); }
        }
    };

    rsx! {
        div { class: "space-y-4",
            div { class: "flex justify-between items-center mb-4",
                h2 { class: "text-2xl font-bold text-gray-900", "{localization.metrics_title()}" }
                button { 
                    class: "px-4 py-2 rounded-lg bg-blue-900 text-white font-semibold hover:bg-blue-800 transition-colors", 
                    onclick: move |_| show_form.set(true), 
                    "{localization.create_metric()}" 
                }
            }

            if show_form() {
                crate::presentation::create_metric_modal::CreateMetricModal {
                    show: show_form,
                    on_success: refresh_trigger,
                    language: language
                }
            }

            div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-5",
                for metric in metrics() {
                    div { key: "{metric.id}", class: "bg-white rounded-lg shadow-md p-5",
                        div { class: "flex justify-between items-start mb-2",
                            div { class: "flex-1",
                                div { class: "flex items-center gap-2 mb-2",
                                    h3 { class: "text-lg font-semibold text-gray-900 m-0", "{metric.name}" }
                                    span { class: "text-xs text-gray-500 px-2 py-1 bg-gray-100 rounded", "{metric.metric_type:?}" }
                                }
                                if let Some(desc) = &metric.description { 
                                    p { class: "text-sm text-gray-600 mb-2", "{desc}" } 
                                }
                                p { class: "text-xs text-gray-500 m-0", "{localization.created_at()} {metric.readable_timestamp()}" }
                            }
                            button {
                                class: "ml-2 px-2 py-1 text-red-600 hover:bg-red-50 rounded transition-colors",
                                title: "{localization.delete_metric()}",
                                onclick: move |_| metric_to_delete.set(Some(metric.id)),
                                "🗑️"
                            }
                        }
                    }
                }
            }

            if let Some(metric_id) = metric_to_delete() {
                div { 
                    class: "fixed inset-0 bg-black bg-opacity-35 flex items-center justify-center p-7 z-50",
                    div { class: "bg-white rounded-lg shadow-xl w-full max-w-md p-6",
                        h3 { class: "text-xl font-bold text-gray-900 mb-4", "{localization.delete_metric_confirm_title()}" }
                        p { class: "text-gray-600 mb-6", 
                            "{localization.delete_metric_confirm_message()}"
                        }
                        div { class: "flex gap-3 justify-end",
                            button { 
                                class: "px-4 py-2 rounded-lg bg-gray-100 text-gray-700 font-semibold hover:bg-gray-200 transition-colors", 
                                onclick: move |_| metric_to_delete.set(None), 
                                "{localization.cancel()}" 
                            }
                            button { 
                                class: "px-4 py-2 rounded-lg bg-red-600 text-white font-semibold hover:bg-red-700 transition-colors", 
                                onclick: move |_| delete_metric_action(metric_id), 
                                "{localization.delete()}" 
                            }
                        }
                    }
                }
            }
        }
    }
}
