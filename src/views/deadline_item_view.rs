use crate::model::datetime::Datetime;
use crate::model::Deadline;
use dioxus::prelude::*;

// --- Continuous color utilities ---
// We map urgency (0..∞) to a continuous gradient through Blue -> Yellow -> Orange -> Red
// using linear RGB interpolation between the stop colors.

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

fn lerp_rgb(a: (u8, u8, u8), b: (u8, u8, u8), t: f32) -> (u8, u8, u8) {
    let t = t.clamp(0.0, 1.0);
    (
        lerp(a.0 as f32, b.0 as f32, t).round() as u8,
        lerp(a.1 as f32, b.1 as f32, t).round() as u8,
        lerp(a.2 as f32, b.2 as f32, t).round() as u8,
    )
}

fn rgb_to_hex(rgb: (u8, u8, u8)) -> String {
    format!("#{:02x}{:02x}{:02x}", rgb.0, rgb.1, rgb.2)
}

/// Continuous color from urgency using piecewise-linear stops.
/// Stops: 0 -> Blue(#3b82f6), 1 -> Yellow(#eab308), 5 -> Orange(#f97316), 10+ -> Red(#ef4444)
fn color_from_urgency_rgb(urgency: f32) -> (u8, u8, u8) {
    let u = urgency.max(0.0);
    let blue = (0x3b, 0x82, 0xf6);
    let yellow = (0xea, 0xb3, 0x08);
    let orange = (0xf9, 0x73, 0x16);
    let red = (0xef, 0x44, 0x44);

    if u <= 1.0 {
        let t = u / 1.0;
        lerp_rgb(blue, yellow, t)
    } else if u <= 5.0 {
        let t = (u - 1.0) / 4.0;
        lerp_rgb(yellow, orange, t)
    } else if u <= 10.0 {
        let t = (u - 5.0) / 5.0;
        lerp_rgb(orange, red, t)
    } else {
        red
    }
}

fn color_from_urgency_hex(urgency: f32) -> String {
    rgb_to_hex(color_from_urgency_rgb(urgency))
}

/// Return a light RGBA tint for the card background based on urgency.
/// We use the base color from `color_from_urgency_rgb` and apply a small alpha.
fn card_tint_from_urgency(urgency: f32) -> String {
    let (r, g, b) = color_from_urgency_rgb(urgency);
    // Choose alpha based on urgency; min 0.03, max 0.16 (subtle tint)
    let mut a = urgency / 20.0; // 0..∞ => 0..∞; but we'll clamp
    if a < 0.03 { a = 0.03 }
    if a > 0.16 { a = 0.16 }
    format!("rgba({}, {}, {}, {:.3})", r, g, b, a)
}

#[component]
pub fn DeadlineItemView(mut deadline: Deadline, mut on_update: EventHandler<Deadline>, mut on_edit: EventHandler<Deadline>, mut on_delete: EventHandler<Deadline>) -> Element {
    // Local, draggable progress state (0-100). If you want to persist upward, we can add a callback later.
    let mut progress = use_signal(|| deadline.progress as f32);
    
    let bar_color = color_from_urgency_hex(deadline.urgency);
    let card_tint = card_tint_from_urgency(deadline.urgency);
    let (r, g, b) = color_from_urgency_rgb(deadline.urgency);
    let border_color = format!("rgba({}, {}, {}, {:.3})", r, g, b, 0.12);
    let due_date_str = deadline.due_date.to_string();
    // Calculate remaining days for the due badge
    let now = Datetime::now();
    let diff = deadline.due_date.time_diff(&now);
    let due_badge = if diff.is_negative {
        format!("Overdue {}d", diff.days)
    } else if diff.days == 0 {
        "Due today".to_string()
    } else {
        format!("Due in {}d", diff.days)
    };
    let progress_width = move || format!("{}%", progress().clamp(0.0, 100.0));
    let edit_clone = deadline.clone();
    let update_clone = deadline.clone();
    let delete_clone = deadline.clone();

    rsx! {
        div {
            class: "card flex flex-col gap-4",
            style: "background-color: {card_tint}; border-color: {border_color};",
            
            // Header
            div {
                class: "flex justify-between items-center",
                div {
                    class: "flex flex-col",
                    h3 { class: "text-xl font-bold", "{deadline.name}" }
                    span { class: "text-sm text-gray-500", "{due_date_str}" }
                }
                div { class: "flex items-center gap-3",
                    // Edit button
                    button {
                        class: "btn-icon",
                        title: "Edit",
                        onclick: move |_| on_edit.call(edit_clone.clone()),
                        svg {
                            width: "20",
                            height: "20",
                            fill: "none",
                            stroke: "currentColor",
                            view_box: "0 0 24 24",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                d: "M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"
                            }
                        }
                    }
                    // Delete button (grouped with Edit)
                    button {
                        class: "btn-icon delete",
                        title: "Delete",
                        onclick: move |_| on_delete.call(delete_clone.clone()),
                        svg {
                            width: "20",
                            height: "20",
                            fill: "none",
                            stroke: "currentColor",
                            view_box: "0 0 24 24",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                // Updated trash icon path (heroicons outline style)
                                d: "M6 19a2 2 0 002 2h8a2 2 0 002-2V7H6v12z M9 7V4h6v3"
                            }
                        }
                    }
                }
            }

            // Badges
            div {
                class: "flex gap-2",
                style: "flex-wrap: wrap;",
                span {
                    class: if diff.is_negative { "badge badge-red" } else { "badge badge-blue" },
                    "{due_badge}"
                }
                span {
                    class: "badge badge-gray",
                    "Difficulty: {deadline.difficulty}"
                }
            }

            // Progress Bar
            div {
                class: "progress-container",
                div {
                    class: "progress-header",
                    div {
                        span { class: "progress-label", "Progress" }
                    }
                    div {
                        span { class: "text-xs font-bold", style: "color: var(--primary-600);", "{progress().round()}%" }
                    }
                }
                div {
                    class: "progress-track",
                    div {
                        class: "progress-fill",
                        style: "width: {progress_width()}; background-color: {bar_color};",
                    }
                    // Input overlay for dragging
                    input {
                        r#type: "range",
                        min: "0",
                        max: "100",
                        step: "1",
                        value: "{progress}",
                        oninput: move |evt| {
                            let val = evt.value().parse::<f32>().unwrap_or(progress());
                           let clamped = val.clamp(0.0, 100.0);
                           progress.set(clamped);
                           let mut d = update_clone.clone();
                           d.progress = clamped.round() as u8;
                           d.update_urgency();
                           on_update.call(d);
                        },
                        class: "range-input",
                    }
                }
            }

            // Tags
            if !deadline.tags.is_empty() {
                div {
                    class: "flex gap-2",
                    style: "flex-wrap: wrap; margin-top: auto;",
                    {
                        deadline.tags.iter().enumerate().map(|(idx, tag)| {
                            rsx! {
                                span {
                                    key: "{idx}",
                                    class: "badge badge-gray",
                                    "{tag}"
                                }
                            }
                        })
                    }
                }
            }
        }
    }
}
