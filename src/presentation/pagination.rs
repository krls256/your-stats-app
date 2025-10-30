use dioxus::prelude::*;
use crate::localization::{get_localization, Language};

#[component]
pub fn Pagination(
    mut current_page: Signal<usize>,
    total_count: usize,
    page_size: usize,
    on_page_change: EventHandler<usize>,
    language: Language,
) -> Element {
    let localization = get_localization(language);
    
    let total_pages = ((total_count.saturating_sub(1)) / page_size) + 1;
    
    let handle_previous = move |_| {
        if current_page() > 1 {
            let new_page = current_page() - 1;
            current_page.set(new_page);
            on_page_change.call(new_page);
        }
    };
    
    let handle_next = move |_| {
        if current_page() * page_size < total_count {
            let new_page = current_page() + 1;
            current_page.set(new_page);
            on_page_change.call(new_page);
        }
    };

    rsx! {
        if total_pages > 1 {
            div { class: "flex items-center justify-between mt-4 pt-4 border-t border-gray-200",
                div { class: "text-sm text-gray-600",
                    "{localization.page_formatted(current_page(), total_pages)}"
                }
                div { class: "flex gap-2",
                    button {
                        class: "px-3 py-1 rounded-lg bg-gray-100 text-gray-700 font-semibold hover:bg-gray-200 disabled:opacity-50 disabled:cursor-not-allowed",
                        disabled: current_page() <= 1,
                        onclick: handle_previous,
                        "{localization.previous()}"
                    }
                    button {
                        class: "px-3 py-1 rounded-lg bg-gray-100 text-gray-700 font-semibold hover:bg-gray-200 disabled:opacity-50 disabled:cursor-not-allowed",
                        disabled: current_page() * page_size >= total_count,
                        onclick: handle_next,
                        "{localization.next()}"
                    }
                }
            }
        }
    }
}
