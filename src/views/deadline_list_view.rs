use crate::model::Deadline;
use crate::views::DeadlineItemView;
use dioxus::prelude::*;

#[derive(PartialEq, Eq, Clone, Copy)]
enum SortType {
    DueDate,
    Urgency,
    Progress,
}

fn sorted_deadlines(input: &[Deadline], sort: SortType) -> Vec<Deadline> {
    let mut v: Vec<Deadline> = input.iter().map(|d| (*d).clone()).collect();
    match sort {
        SortType::DueDate => {
            v.sort_by(|a, b| a.due_date.cmp(&b.due_date));
        }
        SortType::Urgency => {
            // Highest urgency first
            v.sort_by(|a, b| {
                b.urgency
                    .partial_cmp(&a.urgency)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
        }
        SortType::Progress => {
            // Lowest progress first
            v.sort_by(|a, b| a.progress.cmp(&b.progress));
        }
    }
    v
}

#[component]
pub fn DeadlineListView(deadlines: Vec<Deadline>) -> Element {
    let mut sort = use_signal(|| SortType::Urgency);

    let sorted = sorted_deadlines(&deadlines, sort());

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 12px;",

            // Sorting controls
            div {
                style: "display: flex; gap: 8px; align-items: center; flex-wrap: wrap;",
                span { style: "font-weight: 600;", "Sort:" }
                button {
                    onclick: move |_| sort.set(SortType::DueDate),
                    disabled: sort() == SortType::DueDate,
                    "Due date"
                }
                button {
                    onclick: move |_| sort.set(SortType::Urgency),
                    disabled: sort() == SortType::Urgency,
                    "Urgency"
                }
                button {
                    onclick: move |_| sort.set(SortType::Progress),
                    disabled: sort() == SortType::Progress,
                    "Progress"
                }
            }

            // Render items
            div {
                class: "deadline-list",
                { sorted.into_iter().map(|d| {
                    let id = d.id;
                    rsx! {
                        DeadlineItemView {
                            key: "{id}",
                            deadline: d,
                            on_update: move |_| {}
                        }
                    }
                }) }
            }
        }
    }
}
