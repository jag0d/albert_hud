use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

const TITLE_FG: Color = Color::Rgb(255, 223, 128);     // Warm Gold
const STAT_BADGE_BG: Color = Color::Rgb(60, 48, 86);   // Mystic Purple badge
const SELECTED_BG: Color = Color::Rgb(42, 50, 78);     // Highlighted button background
const TEXT_PRIMARY: Color = Color::Rgb(245, 245, 245);
const TEXT_MUTED: Color = Color::Rgb(156, 163, 175);
const ACCENT_YELLOW: Color = Color::Yellow;
const ACCENT_CYAN: Color = Color::Rgb(56, 189, 248);

/// A single stat item (e.g., "Score: 10" or "Modifier: +2")
#[derive(Debug, Clone)]
pub struct StatItem {
    pub label: String,
    pub value: String,
    pub is_accent: bool,
}

/// An interactive button (e.g. "[ 🎲 Roll Check ]")
#[derive(Debug, Clone)]
pub struct ActionButton {
    pub label: String,
    pub is_focused: bool,
}

/// A titled text block (e.g., "Rules & Usage:", "Lore:")
#[derive(Debug, Clone)]
pub struct InspectorSection {
    pub heading: String,
    pub body: String,
}

/// A general-purpose inspector view model
#[derive(Debug, Clone)]
pub struct InspectorView {
    pub title: String,
    pub badge: Option<String>,
    pub stats: Vec<StatItem>,
    pub tags: Vec<String>,
    pub action_rows: Vec<Vec<ActionButton>>,
    pub sections: Vec<InspectorSection>,
}

#[allow(dead_code)]
impl InspectorView {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            badge: None,
            stats: Vec::new(),
            tags: Vec::new(),
            action_rows: Vec::new(),
            sections: Vec::new(),
        }
    }

    pub fn badge(mut self, badge: impl Into<String>) -> Self {
        self.badge = Some(badge.into());
        self
    }

    pub fn stat(mut self, label: impl Into<String>, value: impl Into<String>) -> Self {
        self.stats.push(StatItem {
            label: label.into(),
            value: value.into(),
            is_accent: false,
        });
        self
    }

    pub fn stat_accent(mut self, label: impl Into<String>, value: impl Into<String>) -> Self {
        self.stats.push(StatItem {
            label: label.into(),
            value: value.into(),
            is_accent: true,
        });
        self
    }

    pub fn tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }

    pub fn action(mut self, label: impl Into<String>, is_focused: bool) -> Self {
        self.action_rows.push(vec![ActionButton {
            label: label.into(),
            is_focused,
        }]);
        self
    }

    pub fn action_row<I, S>(mut self, buttons: I) -> Self
    where
        I: IntoIterator<Item = (S, bool)>,
        S: Into<String>,
    {
        let row = buttons
            .into_iter()
            .map(|(label, is_focused)| ActionButton {
                label: label.into(),
                is_focused,
            })
            .collect();
        self.action_rows.push(row);
        self
    }

    pub fn section(mut self, heading: impl Into<String>, body: impl Into<String>) -> Self {
        self.sections.push(InspectorSection {
            heading: heading.into(),
            body: body.into(),
        });
        self
    }

    /// Converts this inspector view model into styled Ratatui Lines for rendering
    pub fn to_lines(&self) -> Vec<Line<'static>> {
        let mut lines = Vec::new();

        // 1. Title + Badge Header
        let mut header_spans = vec![
            Span::styled(
                self.title.clone(),
                Style::default().fg(TITLE_FG).add_modifier(Modifier::BOLD),
            ),
        ];
        if let Some(ref badge) = self.badge {
            header_spans.push(Span::raw(" "));
            header_spans.push(Span::styled(
                format!("({})", badge),
                Style::default().fg(ACCENT_CYAN).add_modifier(Modifier::BOLD),
            ));
        }
        lines.push(Line::from(header_spans));
        lines.push(Line::from(""));

        // 2. Stat Grid (Key-Value Badges)
        if !self.stats.is_empty() {
            let mut stat_spans = Vec::new();
            for (i, stat) in self.stats.iter().enumerate() {
                if i > 0 {
                    stat_spans.push(Span::raw("   |   "));
                }
                stat_spans.push(Span::styled(
                    format!("{}: ", stat.label),
                    Style::default().fg(TEXT_MUTED),
                ));

                let val_style = if stat.is_accent {
                    Style::default()
                        .bg(STAT_BADGE_BG)
                        .fg(TITLE_FG)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                        .fg(TEXT_PRIMARY)
                        .add_modifier(Modifier::BOLD)
                };

                stat_spans.push(Span::styled(format!(" {} ", stat.value), val_style));
            }
            lines.push(Line::from(stat_spans));
            lines.push(Line::from(""));
        }

        // 3. Tags
        if !self.tags.is_empty() {
            let mut tag_spans = vec![
                Span::styled("Tags: ", Style::default().fg(TEXT_MUTED)),
            ];
            for tag in &self.tags {
                tag_spans.push(Span::styled(
                    format!(" [{}] ", tag),
                    Style::default().fg(ACCENT_CYAN),
                ));
            }
            lines.push(Line::from(tag_spans));
            lines.push(Line::from(""));
        }

        // 4. Interactive Action Buttons
        if !self.action_rows.is_empty() {
            lines.push(Line::from(Span::styled(
                "Actions:",
                Style::default().fg(TEXT_MUTED),
            )));

            for row in &self.action_rows {
                let mut row_spans = Vec::new();
                for (i, action) in row.iter().enumerate() {
                    if i > 0 {
                        row_spans.push(Span::raw("   ")); // Spacing between side-by-side buttons
                    }

                    let prefix = if action.is_focused { " ▶ " } else { "   " };
                    let btn_style = if action.is_focused {
                        Style::default()
                            .bg(SELECTED_BG)
                            .fg(ACCENT_YELLOW)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(TEXT_PRIMARY)
                    };

                    row_spans.push(Span::styled(
                        prefix,
                        if action.is_focused {
                            Style::default().fg(ACCENT_YELLOW)
                        } else {
                            Style::default().fg(TEXT_MUTED)
                        },
                    ));
                    row_spans.push(Span::styled(format!("[ {} ]", action.label), btn_style));
                }
                lines.push(Line::from(row_spans));
            }
            lines.push(Line::from(""));
        }

        // 5. Sections / Descriptions
        for section in &self.sections {
            lines.push(Line::from(Span::styled(
                section.heading.clone(),
                Style::default().fg(TEXT_MUTED),
            )));
            lines.push(Line::from(Span::styled(
                section.body.clone(),
                Style::default().fg(TEXT_PRIMARY),
            )));
            lines.push(Line::from(""));
        }

        lines
    }
}
