use crate::table_viewer::TableViewer;

#[derive(Debug, Clone)]
pub struct ChartData {
    pub title: String,
    pub column_name: String,
    pub data_type: String,
    pub items: Vec<ChartItem>,
    pub total_count: usize,
    pub null_count: usize,
    pub unique_count: usize,
}

#[derive(Debug, Clone)]
pub struct ChartItem {
    pub label: String,
    pub count: usize,
    pub percentage: f64,
    pub bar_length: usize,
}

impl TableViewer {
    pub fn generate_chart_data(
        &self,
        max_items: usize,
        bar_width: usize,
        value_to_highlight: &str,
    ) -> Option<ChartData> {
        let stats = self.column_stats.get(&self.current_col)?;
        let column_name = self.data.headers.get(self.current_col)?.clone();
        let data_type = stats
            .data_type
            .clone()
            .unwrap_or_else(|| "UNKNOWN".to_string());
        let total_count = self.data.rows.len();
        let non_null_count = total_count - stats.null_count;

        if stats.value_counts.is_empty() {
            return None;
        }

        let mut sorted_items: Vec<(String, usize)> = stats
            .value_counts
            .iter()
            .map(|(l, c)| (l.clone(), *c))
            .collect();

        sorted_items.sort_by(|a, b| b.1.cmp(&a.1));

        let mut items_to_show: Vec<(String, usize)> = Vec::new();

        let highlighted_item_info = sorted_items
            .iter()
            .enumerate()
            .find(|(_, (label, _))| label == value_to_highlight);

        if sorted_items.len() <= max_items {
            let mut temp_items = sorted_items.clone();
            if let Some((idx, _)) = highlighted_item_info {
                let highlighted = temp_items.remove(idx);
                temp_items.insert(0, highlighted);
            }
            items_to_show = temp_items;
        } else if let Some((idx, item_data)) = highlighted_item_info {
            if idx < max_items - 1 {
                let mut temp_items: Vec<_> =
                    sorted_items.iter().take(max_items - 1).cloned().collect();

                if let Some(pos) = temp_items.iter().position(|(l, _)| l == value_to_highlight) {
                    let highlighted = temp_items.remove(pos);
                    temp_items.insert(0, highlighted);
                }

                let others_count: usize = sorted_items
                    .iter()
                    .skip(max_items - 1)
                    .map(|(_, c)| *c)
                    .sum();

                items_to_show = temp_items;
                items_to_show.push(("<Others>".to_string(), others_count));
            } else {
                items_to_show.push(item_data.clone());

                let other_top_items = sorted_items
                    .iter()
                    .filter(|(label, _)| label != value_to_highlight)
                    .take(max_items - 2)
                    .cloned();

                items_to_show.extend(other_top_items);

                let others_count: usize = stats
                    .value_counts
                    .iter()
                    .filter(|(label, _)| !items_to_show.iter().any(|(l, _)| l == *label))
                    .map(|(_, count)| *count)
                    .sum();

                if others_count > 0 {
                    items_to_show.push(("<Others>".to_string(), others_count));
                }
            }
        }

        let max_count = items_to_show.iter().map(|(_, c)| *c).max().unwrap_or(0);

        let chart_items = items_to_show
            .into_iter()
            .map(|(label, count)| {
                let percentage = if non_null_count > 0 {
                    (count as f64 / non_null_count as f64) * 100.0
                } else {
                    0.0
                };
                let bar_length = if max_count > 0 {
                    ((count as f64 / max_count as f64) * bar_width as f64) as usize
                } else {
                    0
                };
                ChartItem {
                    label,
                    count,
                    percentage,
                    bar_length: bar_length.max(1),
                }
            })
            .collect();

        Some(ChartData {
            title: format!("Distribution of '{}'", column_name),
            column_name,
            data_type,
            items: chart_items,
            total_count,
            null_count: stats.null_count,
            unique_count: stats.unique_count,
        })
    }

    pub fn toggle_chart(&mut self, bar_width: usize) {
        if self.show_chart {
            self.show_chart = false;
            self.chart_data = None;
        } else {
            let max_items = 20;

            let value_to_highlight = self.get_current_cell_value().unwrap_or_default();

            if let Some(chart_data) =
                self.generate_chart_data(max_items, bar_width, &value_to_highlight)
            {
                self.chart_data = Some(chart_data);
                self.show_chart = true;
            }
        }
    }

    pub fn get_chart_display(&self, width: usize) -> Vec<String> {
        if let Some(chart_data) = &self.chart_data {
            let mut lines = Vec::new();

            lines.push(format!("┌─ {} ─┐", chart_data.title));
            lines.push(format!("│ Type: {} │", chart_data.data_type));
            lines.push(format!(
                "│ Total: {}, Unique: {}, Nulls: {} │",
                chart_data.total_count, chart_data.unique_count, chart_data.null_count
            ));
            lines.push("├─────────────────────────────────────────┤".to_string());

            let label_width = width.saturating_sub(25).max(10);
            let bar_char = "█";

            for item in &chart_data.items {
                let truncated_label = if item.label.len() > label_width {
                    format!("{}...", &item.label[..label_width.saturating_sub(3)])
                } else {
                    format!("{:<width$}", item.label, width = label_width)
                };

                let bar = bar_char.repeat(item.bar_length);
                let count_str = format!("{:>6}", item.count);
                let percentage_str = format!("{:>5.1}%", item.percentage);

                lines.push(format!(
                    "│ {} {} {} {} │",
                    truncated_label, bar, count_str, percentage_str
                ));
            }

            lines.push("└─────────────────────────────────────────┘".to_string());
            lines
        } else {
            vec!["No chart data available".to_string()]
        }
    }
}
