use dioxus::prelude::*;
use crate::domain::{correlation::{Correlation, CorrelationType}, MetricType};
use crate::application as app;
use crate::localization::{Language, get_localization};

#[component]
pub fn CorrelationsTab(
    correlations: Signal<Vec<Correlation>>,
    language: Language,
) -> Element {
    let localization = get_localization(language);

    let boolean_corrs: Vec<_> = correlations().iter()
        .filter(|corr| corr.metric1_type == MetricType::Boolean && corr.metric2_type == MetricType::Boolean)
        .cloned()
        .collect();
    let numeric_corrs: Vec<_> = correlations().iter()
        .filter(|corr| corr.metric1_type != MetricType::Boolean && corr.metric2_type != MetricType::Boolean)
        .cloned()
        .collect();

    rsx! {
        div { class: "space-y-4",
            div { class: "flex justify-between items-center mb-4",
                h2 { class: "text-2xl font-bold text-gray-900", "{localization.correlations_title()}" }
                button { 
                    class: "px-4 py-2 rounded-lg bg-blue-900 text-white font-semibold hover:bg-blue-800 transition-colors", 
                    onclick: move |_| {
                        if let Ok(corrs) = app::calculate_correlations() { correlations.set(corrs); }
                    }, 
                    "{localization.calculate_correlations()}" 
                }
            }

            if correlations().is_empty() {
                div { class: "bg-gray-50 rounded-lg p-6 text-center",
                    p { class: "text-gray-600 mb-2", "{localization.click_to_calculate()}" }
                    p { class: "text-gray-500 text-sm", "{localization.need_numeric_metrics()}" }
                }
            } else {
                div { class: "space-y-6",
                    CorrelationLine{
                        correlations: boolean_corrs,
                        title: localization.boolean_metrics(),
                        language: language
                    }

                    CorrelationLine{
                        correlations: numeric_corrs,
                        title: localization.numeric_metrics(),
                        language: language
                    }
                }
            }
        }
    }
}

#[component]
pub fn CorrelationLine(
    correlations: Vec<Correlation>,
    title: String,
    language: Language,
) -> Element {
    let localization = get_localization(language);

    rsx! {
        if !correlations.is_empty() {
            div {
                h3 { class: "text-xl font-bold text-gray-900 mb-4", "{title}" }
                div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-5",
                    for corr in correlations {
                        div { key: "{corr.metric1_name}-{corr.metric2_name}", class: "bg-white rounded-lg shadow-md p-5",
                            div { class: "flex items-center gap-2 mb-3",
                                h4 { class: "text-base font-semibold text-gray-900 m-0", "{corr.metric1_name}" }
                                span { class: "text-gray-400", "↔" }
                                h4 { class: "text-base font-semibold text-gray-900 m-0", "{corr.metric2_name}" }
                            }
                            div { class: "flex items-baseline gap-2 mb-3",
                                span { class: "text-2xl font-bold text-gray-900", "{corr.correlation:.3}" }
                                span { class: "text-sm text-gray-600",
                                    {
                                        match corr.correlation_type() {
                                            CorrelationType::Strong => localization.strong_correlation(),
                                            CorrelationType::Medium => localization.medium_correlation(),
                                            CorrelationType::Weak => localization.weak_correlation() ,
                                        }
                                    }
                                }
                            }
                            div { class: "h-2 bg-gray-200 rounded-full overflow-hidden mb-2",
                                div {
                                    class: "h-full bg-green-500 transition-all",
                                    style: format!("width: {}%", (corr.correlation.abs() * 100.0))
                                }
                            }
                            p { class: "text-xs text-gray-500 m-0", "{localization.based_on_points_formatted(corr.count)}" }
                        }
                    }
                }
            }
        }
    }
}
