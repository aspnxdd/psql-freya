use crate::db::run_query;
use crate::models::{AppChannel, QueryResult};
use freya::icons::lucide;
use freya::prelude::*;
use freya::radio::*;
use regex::Regex;
use std::sync::LazyLock;

static AUTO_QUOTE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(from|join)\s+([a-zA-Z_][a-zA-Z0-9_]*)\.([a-zA-Z_][a-zA-Z0-9_]*)").unwrap()
});

const ROW_HEIGHT: f32 = 32.;
const CELL_PADDING_X: f32 = 12.;

#[derive(PartialEq)]
pub struct MainArea;

impl Component for MainArea {
    fn render(&self) -> impl IntoElement {
        let mut radio = use_radio(AppChannel::Query);
        let error_color = use_theme().read().colors.error;
        let query_text = radio
            .slice_mut_current(|s| &mut s.query_text)
            .into_writable();
        let query_results = radio.read().query_results.clone();
        let error_message = radio.read().error_message.clone();

        let on_run = {
            let mut query_text = query_text.clone();
            move |_| {
                let raw_sql = query_text.read().clone();
                if raw_sql.is_empty() {
                    return;
                }
                let sql = auto_quote_sql(&raw_sql);
                if sql != raw_sql {
                    query_text.set(sql.clone());
                }
                let Some(client) = radio.read().client.clone() else {
                    return;
                };
                spawn(async move {
                    match run_query(&client, &sql).await {
                        Ok(result) => {
                            let mut state = radio.write();
                            state.query_results = Some(result);
                            state.error_message = None;
                        }
                        Err(e) => {
                            radio.write().error_message = Some(format!("Query failed: {e}"));
                        }
                    }
                });
            }
        };

        rect()
            .expanded()
            .height(Size::fill())
            .background(Color::from_rgb(30, 30, 30))
            .padding(Gaps::new_all(12.))
            .child(
                rect()
                    .horizontal()
                    .spacing(8.)
                    .cross_align(Alignment::Center)
                    .content(Content::Flex)
                    .child(
                        Input::new(query_text)
                            .placeholder("SQL query...")
                            .width(Size::flex(1.)),
                    )
                    .child(
                        Button::new()
                            .on_press(on_run)
                            .filled()
                            .padding(Gaps::new(10., 14., 10., 14.))
                            .child(
                                svg(lucide::play())
                                    .theme_color()
                                    .width(Size::px(14.))
                                    .height(Size::px(14.)),
                            ),
                    ),
            )
            .map(error_message, |el, err| {
                el.child(error_banner(&err, error_color))
            })
            .child(results_grid(query_results))
    }
}

fn error_banner(message: &str, color: Color) -> Rect {
    rect()
        .background(Color::from_rgb(80, 30, 30))
        .corner_radius(CornerRadius::new_all(6.))
        .padding(Gaps::new_all(8.))
        .margin(Gaps::new(8., 0., 0., 0.))
        .child(
            label()
                .text(message.to_string())
                .color(color)
                .font_size(12.),
        )
}

fn estimate_text_width(text: &str) -> f32 {
    text.chars().count() as f32 * 6.0
}

fn compute_column_widths(result: &QueryResult) -> Vec<f32> {
    let pad = CELL_PADDING_X * 2.;
    let mut widths: Vec<f32> = result
        .columns
        .iter()
        .map(|col| estimate_text_width(col) + pad)
        .collect();

    for row in &result.rows {
        for (j, cell) in row.iter().enumerate() {
            if let Some(w) = widths.get_mut(j) {
                *w = w.max(estimate_text_width(cell) + pad);
            }
        }
    }

    widths.into_iter().map(|w| w.clamp(80.0, 400.0)).collect()
}

fn results_grid(results: Option<QueryResult>) -> Rect {
    let colors = use_theme().read().colors.clone();
    let Some(result) = results else {
        return rect().expanded().center().child(
            label()
                .text("Run a query or select a table")
                .color(colors.text_placeholder)
                .font_size(14.),
        );
    };

    let widths = compute_column_widths(&result);
    let cols = result.columns;
    let rows = result.rows;
    let row_count = rows.len() + 1;

    rect()
        .expanded()
        .margin(Gaps::new(12., 0., 0., 0.))
        .corner_radius(CornerRadius::new_all(8.))
        .border(
            Border::new()
                .alignment(BorderAlignment::Inner)
                .fill(colors.border)
                .width(1.),
        )
        .overflow(Overflow::Clip)
        .child(
            VirtualScrollView::new_with_data(
                (cols, rows, widths, colors),
                move |i, (cols, rows, widths, colors)| {
                    if i == 0 {
                        header_row(cols, widths, colors).into()
                    } else {
                        data_row(i, &rows[i - 1], widths, colors).into()
                    }
                },
            )
            .length(row_count)
            .item_size(ROW_HEIGHT)
            .expanded(),
        )
}

fn header_row(cols: &[String], widths: &[f32], colors: &ColorsSheet) -> Rect {
    rect()
        .key(0)
        .height(Size::px(ROW_HEIGHT))
        .background(colors.surface_tertiary)
        .horizontal()
        .border(
            Border::new()
                .alignment(BorderAlignment::Inner)
                .fill(colors.border)
                .width(BorderWidth {
                    top: 0.,
                    right: 0.,
                    bottom: 1.,
                    left: 0.,
                }),
        )
        .children(cols.iter().enumerate().map(|(j, col)| {
            cell(j, widths.get(j).copied().unwrap_or(140.))
                .child(tooltipped(
                    col,
                    label()
                        .text(col.clone())
                        .theme_color()
                        .font_size(12.)
                        .max_lines(1)
                        .text_overflow(TextOverflow::Ellipsis),
                ))
                .into_element()
        }))
}

fn data_row(i: usize, row: &[String], widths: &[f32], colors: &ColorsSheet) -> Rect {
    let bg = if i % 2 == 0 {
        colors.surface_primary
    } else {
        colors.surface_secondary
    };
    rect()
        .key(i)
        .height(Size::px(ROW_HEIGHT))
        .background(bg)
        .horizontal()
        .children(row.iter().enumerate().map(|(j, value)| {
            cell(j, widths.get(j).copied().unwrap_or(140.))
                .child(tooltipped(
                    value,
                    label()
                        .text(value.clone())
                        .color(colors.text_secondary)
                        .font_size(12.)
                        .max_lines(1)
                        .text_overflow(TextOverflow::Ellipsis),
                ))
                .into_element()
        }))
}

fn tooltipped(text: &str, content: impl Into<Element>) -> TooltipContainer {
    TooltipContainer::new(Tooltip::new(text.to_string())).child(content)
}

fn cell(j: usize, width: f32) -> Rect {
    rect()
        .key(j)
        .width(Size::px(width))
        .height(Size::fill())
        .padding(Gaps::new(0., CELL_PADDING_X, 0., CELL_PADDING_X))
        .main_align(Alignment::Center)
}

fn auto_quote_sql(sql: &str) -> String {
    AUTO_QUOTE_RE
        .replace_all(sql, |caps: &regex::Captures| {
            let keyword = &caps[1];
            let schema = &caps[2];
            let table = &caps[3];
            format!(r#"{keyword} "{schema}"."{table}""#)
        })
        .to_string()
}
