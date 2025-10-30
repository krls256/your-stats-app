use dioxus::prelude::*;
use crate::domain::MetricType;
use crate::application as app;
use crate::localization::{get_localization, Language};

#[component]
pub fn CreateMetricModal(
    mut show: Signal<bool>,
    mut on_success: Signal<()>,
    language: Language,
) -> Element {
    let localization = get_localization(language);
    
    let mut metric_name = use_signal(String::new);
    let mut metric_type = use_signal(|| String::from(MetricType::default().as_str()));
    let mut metric_description = use_signal(String::new);

    let mut create_metric_action = move || {
        let mtype =  MetricType::from_str(metric_type().as_str()).unwrap_or_default();
        
        let desc = metric_description();
        let desc = if desc.is_empty() { None } else { Some(desc.as_str()) };
        
        if app::create_metric(&metric_name(), mtype, desc).is_ok() {
            metric_name.set(String::new());
            metric_description.set(String::new());
            
            show.set(false);
            on_success.set(());
        }
    };

    let create_metric = move |_| create_metric_action();

    rsx! {
        div { 
            class: "fixed inset-0 bg-black bg-opacity-35 flex items-center justify-center p-7 z-50",
            div { class: "bg-white rounded-lg shadow-xl w-full max-w-2xl p-6",
                div { class: "flex justify-between items-center mb-4",
                    h3 { class: "text-xl font-bold text-gray-900", "{localization.new_metric()}" }
                }
                div { class: "mb-4",
                    label { class: "block text-sm font-medium text-gray-700 mb-1", "{localization.name()}" }
                    input { 
                        class: "w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-900 focus:border-transparent",
                        value: "{metric_name}", 
                        oninput: move |e| metric_name.set(e.value()),
                        onkeydown: move |e| {
                            if e.key() == Key::Enter && !metric_name().is_empty() {
                                create_metric_action();
                            }
                        }
                    }
                }
                div { class: "mb-4",
                    label { class: "block text-sm font-medium text-gray-700 mb-1", "{localization.metric_type()}" }
                    select { 
                        class: "w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-900 focus:border-transparent appearance-none bg-white",
                        value: "{metric_type}", 
                        onchange: move |e| metric_type.set(e.value()),
                        for metric_type in MetricType::all() {
                            option { value: metric_type.as_str(), "{metric_type.as_str()}" }
                        }
                    }
                }
                div { class: "mb-4",
                    label { class: "block text-sm font-medium text-gray-700 mb-1", "{localization.description()}" }
                    textarea { 
                        class: "w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-900 focus:border-transparent",
                        value: "{metric_description}", 
                        oninput: move |e| metric_description.set(e.value()) 
                    }
                }
                div { class: "flex gap-3 justify-end",
                    button { 
                        class: "px-4 py-2 rounded-lg bg-gray-100 text-gray-700 font-semibold hover:bg-gray-200 transition-colors", 
                        onclick: move |_| show.set(false), 
                        "{localization.cancel()}" 
                    }
                    button { 
                        class: "px-4 py-2 rounded-lg bg-blue-900 text-white font-semibold hover:bg-blue-800 transition-colors", 
                        onclick: create_metric, 
                        "{localization.create_with_enter()}" 
                    }
                }
            }
        }
    }
}

