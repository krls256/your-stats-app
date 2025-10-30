mod domain;
mod repository;
mod application;
mod presentation;
mod localization;
mod cli;

use std::sync::OnceLock;
use domain::{correlation::Correlation, metric::Metric};
use dioxus::prelude::*;
use clap::Parser;
use presentation::navigation::Tab::{AddData, Metrics, Correlations};
use presentation::navigation::Tab;
use presentation::language_buttons::{LanguageButtons};
use localization::Language;
use application::{Service};
use anyhow::{Result};


static APP_CTX: OnceLock<Service> = OnceLock::new();
fn main() -> Result<()> {
    let cfg = cli::Cli::parse();

    let srv = Service::new(&cfg.db)?;
    let _ = APP_CTX.set( srv );

    launch(app);

    Ok(())
}

static TABS: &[Tab] = &[
    Metrics,
    AddData,
    Correlations,
];
#[component]
fn app() -> Element {

    let mut metrics = use_signal(Vec::<Metric>::new);
    let correlations = use_signal(Vec::<Correlation>::new);
    let selected_metric = use_signal(|| None::<i64>);
    let error_message = use_signal(|| None::<String>);
    let active_tab = use_signal(|| Metrics);
    let language = use_signal(|| localization::Language::Ua);
    let mut script_loaded = use_signal(|| false);
    let srv = use_signal(|| APP_CTX.get().unwrap());

    use_effect(move || {
        if let Ok(m) = srv.read().list_metrics() { metrics.set(m); }
    });

    rsx! {
        head {
            script {
                src: "https://cdn.tailwindcss.com",
                onload: move |_| script_loaded.set(true)
            }
        }

        if script_loaded() {
            div{}
        }

        div { class: "min-h-screen bg-gray-50",
            div { class: "max-w-6xl mx-auto px-8 py-8",
                div { class: "bg-white rounded-lg shadow-md p-6 mb-6",
                    div { class: "flex justify-between items-center mb-4",
                        h1 { class: "text-3xl font-bold text-gray-900", "{localization::get_localization(language()).app_title()}" }

                        LanguageButtons{
                            language: language,
                            languages: Language::all().collect()
                        }
                    }
                    presentation::navigation::Navigation {
                        tabs: TABS.to_vec(),
                        active_tab: active_tab,
                        language: language()
                    }
                }

                if let Some(err) = error_message() {
                    div { class: "bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded-lg mb-6", "{err}" }
                }

                match active_tab() {
                    presentation::navigation::Tab::Metrics => rsx! { presentation::metrics_tab::MetricsTab { metrics: metrics, language: language(), srv: srv } },
                    presentation::navigation::Tab::AddData => rsx! { presentation::add_data_tab::AddDataTab { metrics: metrics, selected_metric: selected_metric, error_message: error_message, language: language(), srv: srv } },
                    presentation::navigation::Tab::Correlations => rsx! { presentation::correlations_tab::CorrelationsTab { correlations: correlations, language: language(), srv: srv } },
                }
            }
        }
    }
}
