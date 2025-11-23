use crate::model::{datetime::Datetime, Deadline};
use dioxus::prelude::*;
use chrono::{Datelike, NaiveDate};
use std::collections::HashMap;

#[component]
pub fn CalendarView(
    deadlines: Vec<Deadline>,
    on_select_date: EventHandler<Datetime>,
    on_edit_deadline: EventHandler<Deadline>
) -> Element {
    let now = chrono::Local::now();
    let mut current_date = use_signal(|| NaiveDate::from_ymd_opt(now.year(), now.month(), 1).unwrap());

    let year = current_date().year();
    let month = current_date().month();

    // Calculate grid
    // 1st day of month
    let first_day = current_date();
    let days_in_month = if month == 12 {
        NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap().signed_duration_since(first_day).num_days()
    } else {
        NaiveDate::from_ymd_opt(year, month + 1, 1).unwrap().signed_duration_since(first_day).num_days()
    };

    let start_weekday = first_day.weekday().num_days_from_sunday(); // 0 = Sunday
    
    // Group deadlines by date
    let mut deadlines_by_date: HashMap<(u16, u8, u8), Vec<Deadline>> = HashMap::new();
    for d in deadlines {
        let key = (d.due_date.year, d.due_date.month, d.due_date.day);
        deadlines_by_date.entry(key).or_default().push(d);
    }

    rsx! {
        div {
            class: "flex flex-col gap-4 h-full bg-white rounded-xl shadow-sm border border-gray-100 p-4",
            
            // Header: Month/Year and Nav
            div {
                class: "flex justify-between items-center p-2",
                button { 
                    class: "p-1 hover:text-primary-600 text-gray-400 transition-colors", 
                    onclick: move |_| {
                        let mut new_year = year;
                        let mut new_month = month as i32 - 1;
                        while new_month < 1 {
                            new_month += 12;
                            new_year -= 1;
                        }
                        if let Some(d) = NaiveDate::from_ymd_opt(new_year, new_month as u32, 1) {
                            current_date.set(d);
                        }
                    }, 
                    svg {
                        class: "w-6 h-6",
                        fill: "none",
                        view_box: "0 0 24 24",
                        stroke_width: "1.5",
                        stroke: "currentColor",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            d: "M15.75 19.5L8.25 12l7.5-7.5"
                        }
                    }
                }
                h3 { class: "text-lg font-bold text-gray-800", "{first_day.format(\"%B %Y\")}" }
                button { 
                    class: "p-1 hover:text-primary-600 text-gray-400 transition-colors", 
                    onclick: move |_| {
                        let mut new_year = year;
                        let mut new_month = month as i32 + 1;
                        while new_month > 12 {
                            new_month -= 12;
                            new_year += 1;
                        }
                        if let Some(d) = NaiveDate::from_ymd_opt(new_year, new_month as u32, 1) {
                            current_date.set(d);
                        }
                    }, 
                    svg {
                        class: "w-6 h-6",
                        fill: "none",
                        view_box: "0 0 24 24",
                        stroke_width: "1.5",
                        stroke: "currentColor",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            d: "M8.25 4.5l7.5 7.5-7.5 7.5"
                        }
                    }
                }
            }

            // Days Header
            div {
                class: "grid grid-cols-7 gap-1 text-center text-sm font-bold text-gray-500 mb-2",
                div { "Sun" } div { "Mon" } div { "Tue" } div { "Wed" } div { "Thu" } div { "Fri" } div { "Sat" }
            }

            // Calendar Grid
            div {
                class: "grid grid-cols-7 gap-1 auto-rows-fr",
                // Empty cells for start padding
                for _ in 0..start_weekday {
                    div { class: "min-h-[75px] p-2 bg-gray-50/30 rounded-lg" }
                }
                
                // Days
                {
                    (1..=days_in_month).map(|day| {
                        let date_key = (year as u16, month as u8, day as u8);
                        let day_deadlines = deadlines_by_date.get(&date_key).cloned().unwrap_or_default();
                        let is_today = {
                            let n = chrono::Local::now();
                            n.year() == year && n.month() == month && n.day() == day as u32
                        };

                        rsx! {
                            div {
                                class: if is_today { 
                                    "min-h-[75px] p-2 border-2 border-primary-200 bg-primary-50/30 rounded-lg cursor-pointer hover:border-primary-300 transition-colors relative group" 
                                } else { 
                                    "min-h-[75px] p-2 border border-gray-100 hover:border-gray-300 rounded-lg cursor-pointer transition-colors relative group" 
                                },
                                onclick: move |_| {
                                    let dt = Datetime::new(year as u16, month as u8, day as u8, 12, 0);
                                    on_select_date.call(dt);
                                },
                                
                                div { 
                                    class: if is_today { "font-bold text-primary-600 mb-1" } else { "font-medium text-gray-700 mb-1" },
                                    "{day}" 
                                }
                                
                                // Add button that appears on hover (optional, but clicking the cell works too)
                                div {
                                    class: "absolute top-2 right-2 opacity-0 group-hover:opacity-100 text-xs text-gray-400",
                                    "+"
                                }

                                div {
                                    class: "flex flex-col gap-1 overflow-y-auto max-h-[80px] scrollbar-hide",
                                    for d in day_deadlines {
                                        div {
                                            class: "text-xs p-1.5 rounded bg-white border border-gray-200 shadow-sm text-gray-700 truncate hover:bg-primary-50 hover:text-primary-700 hover:border-primary-200 transition-colors",
                                            onclick: move |evt| {
                                                evt.stop_propagation();
                                                on_edit_deadline.call(d.clone());
                                            },
                                            "{d.name}"
                                        }
                                    }
                                }
                            }
                        }
                    })
                }
            }
        }
    }
}
