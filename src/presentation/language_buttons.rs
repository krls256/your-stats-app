use dioxus::prelude::*;
use crate::localization::Language;

#[component]
pub fn LanguageButtons(
    languages: Vec<Language>,
    language: Signal<Language>,
) -> Element {
    rsx! {
        div { class: "flex gap-2",
            for l in languages {
                button {
                    class: if language() == l {
                        "px-3 py-1 rounded bg-blue-900 text-white text-sm"
                    } else {
                        "px-3 py-1 rounded bg-gray-200 text-gray-700 text-sm hover:bg-gray-300"
                    },
                    onclick: move |_| language.set(l),
                    "{l.as_str()}"
                }
            }
        }
    }
}