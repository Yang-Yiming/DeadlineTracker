use dioxus::prelude::*;
use crate::model::Deadline;

// --- Continuous color utilities ---
// We map urgency (0..∞) to a continuous gradient through Blue -> Yellow -> Orange -> Red
// using linear RGB interpolation between the stop colors.

fn lerp(a: f32, b: f32, t: f32) -> f32 { a + (b - a) * t }

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

fn rgb_to_rgba(rgb: (u8, u8, u8), a: f32) -> String {
    format!("rgba({}, {}, {}, {})", rgb.0, rgb.1, rgb.2, a)
}

fn darken_rgb(rgb: (u8, u8, u8), amount: f32) -> (u8, u8, u8) {
    let amt = amount.clamp(0.0, 1.0);
    (
        (rgb.0 as f32 * (1.0 - amt)).round() as u8,
        (rgb.1 as f32 * (1.0 - amt)).round() as u8,
        (rgb.2 as f32 * (1.0 - amt)).round() as u8,
    )
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

fn color_from_urgency_bg_rgba(urgency: f32, alpha: f32) -> String {
    rgb_to_rgba(color_from_urgency_rgb(urgency), alpha)
}

#[component]
pub fn DeadlineItemView(deadline: Deadline) -> Element {
    // Local, draggable progress state (0-100). If you want to persist upward, we can add a callback later.
    let mut progress = use_signal(|| deadline.progress as f32);
    // Track hovered milestone index to show a custom tooltip above the marker
    let mut hovered_marker = use_signal(|| None as Option<usize>);

    let bar_color = color_from_urgency_hex(deadline.urgency);
    let border_color = color_from_urgency_hex(deadline.urgency);
    let bg_color = color_from_urgency_bg_rgba(deadline.urgency, 0.10);
    let due_date_str = deadline.due_date.to_string();
    let progress_width = move || format!("{}%", progress().clamp(0.0, 100.0));

    rsx! {
        div {
            style: "
                border: 2px solid;
                border-color: {border_color};
                background-color: {bg_color};
                border-radius: 8px;
                padding: 16px;
                margin: 12px 0;
                display: flex;
                flex-direction: column;
                gap: 12px;
            ",
            // Header with title (urgency hidden as requested)
            div {
                style: "display: flex; justify-content: space-between; align-items: center;",
                h3 {
                    style: "margin: 0; font-size: 1.25rem; font-weight: 600;",
                    "{deadline.name}"
                }
            }
            
            // Due date
            div {
                style: "font-size: 0.95rem; opacity: 0.9;",
                "📅 {due_date_str}"
            }
            
            // Progress bar (draggable) with milestone markers
            div {
                style: "display: flex; flex-direction: column; gap: 6px;",
                // Slider container (relative) so we can absolutely position markers
                div {
                    style: "
                        position: relative;
                        width: 100%;
                        height: 16px;
                    ",
                    // Visible custom track
                    div {
                        style: "
                            position: absolute;
                            inset: 4px 0; /* center vertically with small padding */
                            height: 8px;
                            border-radius: 4px;
                            background-color: rgba(255, 255, 255, 0.12);
                            overflow: hidden;
                            z-index: 0;
                        ",
                        div {
                            style: "
                                width: {progress_width()};
                                height: 100%;
                                background-color: {bar_color};
                                transition: width 0.06s linear;
                            "
                        }
                    }

                    // Milestone markers over the bar
                    {
                        deadline.milestones.iter().enumerate().map(|(idx, (pct, desc))| {
                            let left = (*pct as f32).clamp(0.0, 100.0);
                            let marker_rgb = darken_rgb(color_from_urgency_rgb(deadline.urgency), 0.4);
                            let marker_color = rgb_to_hex(marker_rgb);
                            let desc_text = desc.clone();
                            let pct_value = *pct as f32;
                            rsx! {
                                // Wrapper hit-area to ensure hover shows the tooltip above the input overlay
                                div {
                                    key: "{idx}",
                                    style: "
                                        position: absolute;
                                        left: calc({left}% - 10px);
                                        top: 0px;
                                        width: 20px; /* larger hover/click area */
                                        height: 20px;
                                        z-index: 2;
                                        background: transparent;
                                        cursor: pointer;
                                    ",
                                    onmouseenter: move |_| hovered_marker.set(Some(idx)),
                                    onmouseleave: move |_| hovered_marker.set(None),
                                    onclick: move |_| progress.set(pct_value),

                                    // Actual thin marker centered inside
                                    div {
                                        style: "
                                            position: absolute;
                                            left: 50%;
                                            top: 2px;
                                            transform: translateX(-1px);
                                            width: 2px;
                                            height: 12px;
                                            background-color: {marker_color};
                                            border-radius: 1px;
                                            opacity: 0.98;
                                            box-shadow: 0 0 0 1px rgba(0,0,0,0.2);
                                            pointer-events: none; /* let wrapper receive hover/click */
                                        "
                                    }

                                    // Tooltip above the marker when hovered
                                    {
                                        if hovered_marker() == Some(idx) {
                                            rsx! {
                                                div {
                                                    style: "
                                                        position: absolute;
                                                        left: 50%;
                                                        top: -6px;
                                                        transform: translate(-50%, -100%);
                                                        padding: 2px 6px;
                                                        background: rgba(0,0,0,0.8);
                                                        color: #fff;
                                                        font-size: 0.75rem;
                                                        line-height: 1rem;
                                                        border: 1px solid {marker_color};
                                                        border-radius: 4px;
                                                        white-space: nowrap;
                                                        z-index: 3;
                                                        pointer-events: none;
                                                    ",
                                                    "{desc_text}"
                                                }
                                            }
                                        } else {
                                            rsx! { div { } }
                                        }
                                    }
                                }
                            }
                        })
                    }

                    // Invisible range input overlay to capture drag
                    input {
                        r#type: "range",
                        min: "0",
                        max: "100",
                        step: "1",
                        value: "{progress}",
                        oninput: move |evt| {
                            let val = evt.value().parse::<f32>().unwrap_or(progress());
                            progress.set(val);
                        },
                        style: "
                            position: absolute;
                            inset: 0;
                            width: 100%;
                            height: 16px;
                            opacity: 0; /* keep it invisible but interactive */
                            cursor: pointer;
                            z-index: 1;
                        ",
                        aria_label: "Progress"
                    }
                }

                // Progress text on the right
                span {
                    style: "font-size: 0.85rem; text-align: right; font-weight: 500;",
                    "{(progress() as i32)}%"
                }
            }
            
            // Tags (if any)
            if !deadline.tags.is_empty() {
                div {
                    style: "display: flex; flex-wrap: wrap; gap: 8px;",
                    {
                        deadline.tags.iter().enumerate().map(|(idx, tag)| {
                            rsx! {
                                span {
                                    key: "{idx}",
                                    style: "
                                        background-color: rgba(255, 255, 255, 0.1);
                                        padding: 4px 10px;
                                        border-radius: 12px;
                                        font-size: 0.8rem;
                                    ",
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