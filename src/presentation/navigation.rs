use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tab {
    Metrics,
    AddData,
    Correlations,
}

impl Tab {
    pub fn as_str(&self) -> &'static str {
        match self {
            Tab::Metrics => "metrics",
            Tab::AddData => "add-data",
            Tab::Correlations => "correlations",
        }
    }

    pub fn label(&self, localization: &'static (dyn crate::localization::LocalizationTrait + Send + Sync)) -> &'static str {
        match self {
            Tab::Metrics => localization.tab_metrics(),
            Tab::AddData => localization.tab_add_data(),
            Tab::Correlations => localization.tab_correlations(),
        }
    }
}

#[component]
pub fn Navigation(
    tabs: Vec<Tab>,
    active_tab: Signal<Tab>,
    language: crate::localization::Language,
) -> Element {
    let loc = crate::localization::get_localization(language);
    rsx! {
        div { class: "flex gap-3",
            for tab in tabs {
                button {
                    key: "{tab:?}",
                    class: if active_tab() == tab {
                        "px-4 py-2 rounded-lg bg-blue-900 text-white font-semibold"
                    } else {
                        "px-4 py-2 rounded-lg bg-gray-100 text-gray-700 font-semibold hover:bg-gray-200"
                    },
                    onclick: move |_| active_tab.set(tab),
                    "{tab.label(loc)}"
                }
            }
        }
    }
}

