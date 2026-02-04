//! UI rendering for the TUI

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, Paragraph, Tabs, Wrap},
};

use super::state::{ActionTypeSelection, AppState, LogLevel, Mode, RuleEditorField, SettingsItem, View};
use crate::config::Config;
use crate::theme::Theme;

/// ASCII art logo for Tidy
const LOGO: &str = r#"
  ████████╗██╗██████╗ ██╗   ██╗
  ╚══██╔══╝██║██╔══██╗╚██╗ ██╔╝
     ██║   ██║██║  ██║ ╚████╔╝ 
     ██║   ██║██║  ██║  ╚██╔╝  
     ██║   ██║██████╔╝   ██║   
     ╚═╝   ╚═╝╚═════╝    ╚═╝   
"#;

/// Small broom icon
const BROOM: &str = "🧹";

/// Render the entire UI
pub fn render(frame: &mut Frame, state: &AppState) {
    let colors = state.theme.colors();

    // Set background
    let area = frame.area();
    let bg_block = Block::default().style(Style::default().bg(colors.bg));
    frame.render_widget(bg_block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Tabs
            Constraint::Min(0),    // Main content
            Constraint::Length(1), // Status bar
        ])
        .split(area);

    render_tabs(frame, state, chunks[0]);
    render_main(frame, state, chunks[1]);
    render_status_bar(frame, state, chunks[2]);

    // Render help popup if active
    if state.show_help || state.mode == Mode::Help {
        render_help_popup(frame, state);
    }

    // Render theme picker if active
    if state.mode == Mode::ThemePicker {
        render_theme_picker(frame, state);
    }

    // Render settings dialog if active
    if state.mode == Mode::Settings {
        render_settings_dialog(frame, state);
    }

    // Render rule editor if active
    if matches!(state.mode, Mode::EditRule | Mode::AddRule) {
        render_rule_editor(frame, state);
    }
}

fn render_tabs(frame: &mut Frame, state: &AppState, area: Rect) {
    let colors = state.theme.colors();

    let titles: Vec<Line> = vec![
        format!(
            "{}  Dashboard",
            if state.view == View::Dashboard {
                "●"
            } else {
                "○"
            }
        ),
        format!(
            "{}  Rules",
            if state.view == View::Rules {
                "●"
            } else {
                "○"
            }
        ),
        format!(
            "{}  Watches",
            if state.view == View::Watches {
                "●"
            } else {
                "○"
            }
        ),
        format!(
            "{}  Log",
            if state.view == View::Log {
                "●"
            } else {
                "○"
            }
        ),
    ]
    .into_iter()
    .map(Line::from)
    .collect();

    let selected = match state.view {
        View::Dashboard => 0,
        View::Rules => 1,
        View::Watches => 2,
        View::Log => 3,
    };

    let tabs = Tabs::new(titles)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(colors.block())
                .title(format!(" {} Tidy ", BROOM))
                .title_style(colors.logo_style_primary()),
        )
        .select(selected)
        .style(colors.tab())
        .highlight_style(colors.tab_active())
        .divider(Span::styled(" │ ", colors.text_muted()));

    frame.render_widget(tabs, area);
}

fn render_main(frame: &mut Frame, state: &AppState, area: Rect) {
    match state.view {
        View::Dashboard => render_dashboard(frame, state, area),
        View::Rules => render_rules(frame, state, area),
        View::Watches => render_watches(frame, state, area),
        View::Log => render_log(frame, state, area),
    }
}

fn render_dashboard(frame: &mut Frame, state: &AppState, area: Rect) {
    let colors = state.theme.colors();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(9), // Logo
            Constraint::Min(0),    // Content
        ])
        .split(area);

    // Logo
    let logo_lines: Vec<Line> = LOGO
        .lines()
        .enumerate()
        .map(|(i, line)| {
            // Gradient effect
            let style = if i % 2 == 0 {
                colors.logo_style_primary()
            } else {
                colors.logo_style_secondary()
            };
            Line::styled(line, style)
        })
        .collect();

    let logo = Paragraph::new(logo_lines)
        .alignment(Alignment::Center)
        .block(Block::default());
    frame.render_widget(logo, chunks[0]);

    // Content area
    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    // Left: Stats
    let enabled_rules = state.config.rules.iter().filter(|r| r.enabled).count();
    let total_rules = state.config.rules.len();
    let watch_count = state.config.watches.len();

    let stats_content = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("  📁 Watch Folders:  ", colors.text_dim()),
            Span::styled(watch_count.to_string(), colors.text_primary()),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  📋 Total Rules:    ", colors.text_dim()),
            Span::styled(total_rules.to_string(), colors.text_primary()),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  ✓  Active Rules:   ", colors.text_dim()),
            Span::styled(enabled_rules.to_string(), colors.text_success()),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  ✗  Disabled:       ", colors.text_dim()),
            Span::styled(
                (total_rules - enabled_rules).to_string(),
                colors.text_muted(),
            ),
        ]),
        Line::from(""),
        Line::from(""),
        Line::from(vec![
            Span::styled("  🔌 Daemon:         ", colors.text_dim()),
            Span::styled("Not connected", colors.text_error()),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  🎨 Theme:          ", colors.text_dim()),
            Span::styled(state.theme.name(), colors.text_secondary()),
        ]),
    ];

    let stats = Paragraph::new(stats_content).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(colors.block())
            .title(" Statistics ")
            .title_style(colors.text_primary()),
    );
    frame.render_widget(stats, content_chunks[0]);

    // Right: Quick Actions
    let actions_content = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("  ", Style::default()),
            Span::styled("[r]", colors.key_hint()),
            Span::styled(" View & manage rules", colors.text()),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  ", Style::default()),
            Span::styled("[w]", colors.key_hint()),
            Span::styled(" Watch folders", colors.text()),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  ", Style::default()),
            Span::styled("[l]", colors.key_hint()),
            Span::styled(" Activity log", colors.text()),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  ", Style::default()),
            Span::styled("[s]", colors.key_hint()),
            Span::styled(" Settings", colors.text()),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  ", Style::default()),
            Span::styled("[?]", colors.key_hint()),
            Span::styled(" Help & keybindings", colors.text()),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  ", Style::default()),
            Span::styled("[t]", colors.key_hint()),
            Span::styled(" Change theme", colors.text()),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  ", Style::default()),
            Span::styled("[q]", colors.key_hint()),
            Span::styled(" Quit", colors.text()),
        ]),
    ];

    let actions = Paragraph::new(actions_content).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(colors.block())
            .title(" Quick Actions ")
            .title_style(colors.text_primary()),
    );
    frame.render_widget(actions, content_chunks[1]);
}

fn render_rules(frame: &mut Frame, state: &AppState, area: Rect) {
    let colors = state.theme.colors();

    if state.config.rules.is_empty() {
        let empty = Paragraph::new(vec![
            Line::from(""),
            Line::from(""),
            Line::styled("  No rules configured", colors.text_muted()),
            Line::from(""),
            Line::from(vec![
                Span::styled("  Edit ", colors.text_dim()),
                Span::styled("~/.config/tidy/config.toml", colors.text_primary()),
                Span::styled(" to add rules", colors.text_dim()),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("  Or press ", colors.text_dim()),
                Span::styled("[n]", colors.key_hint()),
                Span::styled(" to create a new rule", colors.text_dim()),
            ]),
        ])
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(colors.block())
                .title(" Rules ")
                .title_style(colors.text_primary()),
        );
        frame.render_widget(empty, area);
        return;
    }

    let items: Vec<ListItem> = state
        .config
        .rules
        .iter()
        .enumerate()
        .map(|(i, rule)| {
            let (status_icon, status_style) = if rule.enabled {
                ("✓", colors.text_success())
            } else {
                ("✗", colors.text_error())
            };

            let is_selected = state.selected_rule == Some(i);
            let base_style = if is_selected {
                colors.selected()
            } else {
                colors.text()
            };

            // Build the rule line
            let action_preview = match &rule.action {
                crate::rules::Action::Move { destination, .. } => {
                    format!("→ {}", destination.display())
                }
                crate::rules::Action::Copy { destination, .. } => {
                    format!("⇒ {}", destination.display())
                }
                crate::rules::Action::Rename { pattern } => format!("✎ {}", pattern),
                crate::rules::Action::Trash => "🗑 Trash".to_string(),
                crate::rules::Action::Delete => "⚠ Delete".to_string(),
                crate::rules::Action::Run { command, .. } => format!("$ {}", command),
                crate::rules::Action::Archive { .. } => "📦 Archive".to_string(),
                crate::rules::Action::Nothing => "∅ Nothing".to_string(),
            };

            ListItem::new(Line::from(vec![
                Span::styled(format!(" {} ", status_icon), status_style),
                Span::styled(&rule.name, base_style.add_modifier(Modifier::BOLD)),
                Span::styled(format!("  {}", action_preview), colors.text_dim()),
            ]))
            .style(base_style)
        })
        .collect();

    let rules_list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(if state.view == View::Rules {
                    colors.block_focus()
                } else {
                    colors.block()
                })
                .title(format!(" Rules ({}) ", state.config.rules.len()))
                .title_style(colors.text_primary()),
        )
        .highlight_style(colors.selected());

    frame.render_widget(rules_list, area);
}

fn render_watches(frame: &mut Frame, state: &AppState, area: Rect) {
    let colors = state.theme.colors();

    if state.config.watches.is_empty() {
        let empty = Paragraph::new(vec![
            Line::from(""),
            Line::from(""),
            Line::styled("  No watch folders configured", colors.text_muted()),
            Line::from(""),
            Line::from(vec![
                Span::styled("  Edit ", colors.text_dim()),
                Span::styled("~/.config/tidy/config.toml", colors.text_primary()),
                Span::styled(" to add folders", colors.text_dim()),
            ]),
        ])
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(colors.block())
                .title(" Watch Folders ")
                .title_style(colors.text_primary()),
        );
        frame.render_widget(empty, area);
        return;
    }

    let items: Vec<ListItem> = state
        .config
        .watches
        .iter()
        .enumerate()
        .map(|(i, watch)| {
            let is_selected = state.selected_watch == Some(i);
            let base_style = if is_selected {
                colors.selected()
            } else {
                colors.text()
            };

            let recursive_indicator = if watch.recursive { " (recursive)" } else { "" };
            let path_str = watch.path.display().to_string();

            // Check if path exists
            let (icon, path_style) = if watch.path.exists() {
                ("📁", colors.text())
            } else {
                ("⚠", colors.text_warning())
            };

            ListItem::new(Line::from(vec![
                Span::styled(format!(" {} ", icon), base_style),
                Span::styled(path_str, path_style),
                Span::styled(recursive_indicator, colors.text_muted()),
            ]))
            .style(base_style)
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(if state.view == View::Watches {
                colors.block_focus()
            } else {
                colors.block()
            })
            .title(format!(" Watch Folders ({}) ", state.config.watches.len()))
            .title_style(colors.text_primary()),
    );

    frame.render_widget(list, area);
}

fn render_log(frame: &mut Frame, state: &AppState, area: Rect) {
    let colors = state.theme.colors();

    if state.log_entries.is_empty() {
        let empty = Paragraph::new(vec![
            Line::from(""),
            Line::from(""),
            Line::styled("  No activity yet", colors.text_muted()),
            Line::from(""),
            Line::styled("  Waiting for file events...", colors.text_dim()),
        ])
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(colors.block())
                .title(" Activity Log ")
                .title_style(colors.text_primary()),
        );
        frame.render_widget(empty, area);
        return;
    }

    let items: Vec<ListItem> = state
        .log_entries
        .iter()
        .rev()
        .map(|entry| {
            let (icon, level_style) = match entry.level {
                LogLevel::Info => ("ℹ", colors.text_info()),
                LogLevel::Success => ("✓", colors.text_success()),
                LogLevel::Warning => ("⚠", colors.text_warning()),
                LogLevel::Error => ("✗", colors.text_error()),
            };

            let time = entry.timestamp.format("%H:%M:%S").to_string();

            ListItem::new(Line::from(vec![
                Span::styled(format!(" {} ", icon), level_style),
                Span::styled(format!("[{}] ", time), colors.text_muted()),
                Span::styled(&entry.message, colors.text()),
            ]))
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(if state.view == View::Log {
                colors.block_focus()
            } else {
                colors.block()
            })
            .title(format!(
                " Activity Log ({}) [c: clear] ",
                state.log_entries.len()
            ))
            .title_style(colors.text_primary()),
    );

    frame.render_widget(list, area);
}

fn render_status_bar(frame: &mut Frame, state: &AppState, area: Rect) {
    let colors = state.theme.colors();

    let content = if let Some(ref msg) = state.status_message {
        vec![
            Span::styled(" ", Style::default()),
            Span::styled(msg, colors.text_secondary()),
        ]
    } else {
        vec![
            Span::styled(" ", Style::default()),
            Span::styled("Tab", colors.key_hint()),
            Span::styled(": switch views  ", colors.text_muted()),
            Span::styled("?", colors.key_hint()),
            Span::styled(": help  ", colors.text_muted()),
            Span::styled("s", colors.key_hint()),
            Span::styled(": settings  ", colors.text_muted()),
            Span::styled("t", colors.key_hint()),
            Span::styled(": theme  ", colors.text_muted()),
            Span::styled("q", colors.key_hint()),
            Span::styled(": quit", colors.text_muted()),
        ]
    };

    let status =
        Paragraph::new(Line::from(content)).style(Style::default().bg(colors.bg_secondary));
    frame.render_widget(status, area);
}

fn render_help_popup(frame: &mut Frame, state: &AppState) {
    let colors = state.theme.colors();
    let area = frame.area();

    // Calculate popup size
    let popup_width = 60u16.min(area.width.saturating_sub(4));
    let popup_height = 22u16.min(area.height.saturating_sub(4));

    let popup_area = Rect {
        x: (area.width - popup_width) / 2,
        y: (area.height - popup_height) / 2,
        width: popup_width,
        height: popup_height,
    };

    // Clear the area
    frame.render_widget(Clear, popup_area);

    let help_content = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            "  Navigation",
            colors.text_primary().add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![
            Span::styled("  Tab / Shift+Tab    ", colors.key_hint()),
            Span::styled("Switch between views", colors.text()),
        ]),
        Line::from(vec![
            Span::styled("  1-4                ", colors.key_hint()),
            Span::styled("Jump to view directly", colors.text()),
        ]),
        Line::from(vec![
            Span::styled("  j/k or ↑/↓         ", colors.key_hint()),
            Span::styled("Navigate lists", colors.text()),
        ]),
        Line::from(vec![
            Span::styled("  g/G                ", colors.key_hint()),
            Span::styled("Go to first/last item", colors.text()),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "  Rules",
            colors.text_primary().add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![
            Span::styled("  Enter/Space        ", colors.key_hint()),
            Span::styled("Toggle rule on/off", colors.text()),
        ]),
        Line::from(vec![
            Span::styled("  e                  ", colors.key_hint()),
            Span::styled("Edit selected rule", colors.text()),
        ]),
        Line::from(vec![
            Span::styled("  n                  ", colors.key_hint()),
            Span::styled("Create new rule", colors.text()),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "  General",
            colors.text_primary().add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![
            Span::styled("  s                  ", colors.key_hint()),
            Span::styled("Open settings", colors.text()),
        ]),
        Line::from(vec![
            Span::styled("  t                  ", colors.key_hint()),
            Span::styled("Open theme selector", colors.text()),
        ]),
        Line::from(vec![
            Span::styled("  ?                  ", colors.key_hint()),
            Span::styled("Toggle this help", colors.text()),
        ]),
        Line::from(vec![
            Span::styled("  q / Ctrl+c         ", colors.key_hint()),
            Span::styled("Quit application", colors.text()),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Press ", colors.text_muted()),
            Span::styled("Esc", colors.key_hint()),
            Span::styled(" or ", colors.text_muted()),
            Span::styled("?", colors.key_hint()),
            Span::styled(" to close", colors.text_muted()),
        ]),
    ];

    let help = Paragraph::new(help_content)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(colors.block_focus())
                .style(Style::default().bg(colors.bg_secondary))
                .title(" ⌨ Keyboard Shortcuts ")
                .title_style(colors.text_primary()),
        )
        .wrap(Wrap { trim: false });

    frame.render_widget(help, popup_area);
}

/// Helper to create a centered rect
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_width = r.width * percent_x / 100;
    let popup_height = r.height * percent_y / 100;
    Rect {
        x: r.x + (r.width - popup_width) / 2,
        y: r.y + (r.height - popup_height) / 2,
        width: popup_width,
        height: popup_height,
    }
}

fn render_theme_picker(frame: &mut Frame, state: &AppState) {
    let colors = state.theme.colors();
    let area = frame.area();

    let popup_area = centered_rect(50, 70, area);
    frame.render_widget(Clear, popup_area);

    let themes = Theme::all();
    let items: Vec<ListItem> = themes
        .iter()
        .enumerate()
        .map(|(i, theme)| {
            let palette = theme.palette();
            let selected = i == state.theme_picker_index;

            // Create color preview squares
            let preview = format!(
                "  {} {} ",
                if selected { "▸" } else { " " },
                theme.name()
            );

            let style = if selected {
                Style::default()
                    .fg(palette.accent)
                    .bg(palette.selection)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(palette.fg)
            };

            ListItem::new(Line::from(vec![
                Span::styled(preview, style),
                Span::styled("█", Style::default().fg(palette.accent)),
                Span::styled("█", Style::default().fg(palette.secondary)),
                Span::styled("█", Style::default().fg(palette.success)),
                Span::styled("█", Style::default().fg(palette.warning)),
            ]))
        })
        .collect();

    let theme_list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(colors.primary))
            .border_type(BorderType::Rounded)
            .style(Style::default().bg(colors.bg))
            .title(format!(
                " 🎨 Select Theme ({}/{}) ",
                state.theme_picker_index + 1,
                themes.len()
            ))
            .title_bottom(Line::from(" ↑↓ navigate │ ↵ apply │ Esc cancel ").centered()),
    );

    frame.render_widget(theme_list, popup_area);
}

fn render_settings_dialog(frame: &mut Frame, state: &AppState) {
    let colors = state.theme.colors();
    let area = frame.area();

    // Calculate popup size - a bit wider for settings
    let popup_width = 60u16.min(area.width.saturating_sub(4));
    let popup_height = 18u16.min(area.height.saturating_sub(4));

    let popup_area = Rect {
        x: (area.width - popup_width) / 2,
        y: (area.height - popup_height) / 2,
        width: popup_width,
        height: popup_height,
    };

    // Clear the area
    frame.render_widget(Clear, popup_area);

    let items = SettingsItem::all();
    let list_items: Vec<ListItem> = items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let selected = i == state.settings_index;
            let cursor = if selected { "▸" } else { " " };

            let value_str = get_settings_value_display(state, *item);

            let style = if selected {
                colors.selected().add_modifier(Modifier::BOLD)
            } else {
                colors.text()
            };

            let value_style = if selected {
                colors.text_secondary()
            } else {
                colors.text_muted()
            };

            ListItem::new(Line::from(vec![
                Span::styled(format!(" {} ", cursor), style),
                Span::styled(format!("{} ", item.icon()), style),
                Span::styled(format!("{:<24}", item.label()), style),
                Span::styled(value_str, value_style),
            ]))
        })
        .collect();

    let settings_list = List::new(list_items).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(colors.primary))
            .border_type(BorderType::Rounded)
            .style(Style::default().bg(colors.bg))
            .title(format!(
                " ⚙ Settings ({}/{}) ",
                state.settings_index + 1,
                items.len()
            ))
            .title_style(colors.text_primary())
            .title_bottom(Line::from(" ↑↓ navigate │ ↵/␣ toggle │ ←→ adjust │ Esc close ").centered()),
    );

    frame.render_widget(settings_list, popup_area);
}

fn get_settings_value_display(state: &AppState, item: SettingsItem) -> String {
    match item {
        SettingsItem::DaemonControl => {
            if state.daemon_running {
                "● Running".to_string()
            } else {
                "○ Stopped".to_string()
            }
        }
        SettingsItem::ThemeSelection => state.theme.name().to_string(),
        SettingsItem::ConfigLocation => {
            state
                .config_path
                .as_ref()
                .map(|p| {
                    // Shorten path for display
                    let s = p.display().to_string();
                    if s.len() > 25 {
                        format!("...{}", &s[s.len() - 22..])
                    } else {
                        s
                    }
                })
                .or_else(|| {
                    Config::default_path().map(|p| {
                        let s = p.display().to_string();
                        if s.len() > 25 {
                            format!("...{}", &s[s.len() - 22..])
                        } else {
                            s
                        }
                    })
                })
                .unwrap_or_else(|| "Not set".to_string())
        }
        SettingsItem::PollingInterval => {
            format!("{}s", state.config.general.polling_interval_secs)
        }
        SettingsItem::LogRetention => {
            format!("{} entries", state.config.general.log_retention)
        }
        SettingsItem::StartupBehavior => {
            if state.config.general.start_daemon_on_launch {
                "✓ Enabled".to_string()
            } else {
                "✗ Disabled".to_string()
            }
        }
        SettingsItem::Notifications => {
            let status = if state.config.general.notifications_enabled {
                "✓ Enabled"
            } else {
                "✗ Disabled"
            };
            format!("{} (coming soon)", status)
        }
    }
}

fn render_rule_editor(frame: &mut Frame, state: &AppState) {
    let colors = state.theme.colors();
    let area = frame.area();

    let Some(ref editor) = state.rule_editor else {
        return;
    };

    // Calculate popup size - larger for the editor
    let popup_width = 70u16.min(area.width.saturating_sub(4));
    let popup_height = 30u16.min(area.height.saturating_sub(4));

    let popup_area = Rect {
        x: (area.width - popup_width) / 2,
        y: (area.height - popup_height) / 2,
        width: popup_width,
        height: popup_height,
    };

    // Clear the area
    frame.render_widget(Clear, popup_area);

    let title = if state.mode == Mode::AddRule {
        " ➕ New Rule "
    } else {
        " ✏ Edit Rule "
    };

    // Build the form content
    let mut lines: Vec<Line> = Vec::new();

    // Helper to create a field line
    let field_line = |label: &str, value: &str, field: RuleEditorField, current: RuleEditorField| -> Line {
        let is_focused = field == current;
        let cursor = if is_focused { "▸" } else { " " };
        let label_style = if is_focused {
            colors.text_primary().add_modifier(Modifier::BOLD)
        } else {
            colors.text_dim()
        };
        let value_style = if is_focused {
            colors.text_secondary()
        } else {
            colors.text()
        };
        let input_display = if is_focused && value.is_empty() {
            "│".to_string()
        } else if is_focused {
            format!("{}│", value)
        } else if value.is_empty() {
            "(empty)".to_string()
        } else {
            value.to_string()
        };

        Line::from(vec![
            Span::styled(format!(" {} ", cursor), label_style),
            Span::styled(format!("{:<20}", label), label_style),
            Span::styled(input_display, value_style),
        ])
    };

    // Helper for toggle fields
    let toggle_line = |label: &str, value: bool, field: RuleEditorField, current: RuleEditorField| -> Line {
        let is_focused = field == current;
        let cursor = if is_focused { "▸" } else { " " };
        let label_style = if is_focused {
            colors.text_primary().add_modifier(Modifier::BOLD)
        } else {
            colors.text_dim()
        };
        let value_display = if value { "✓ Yes" } else { "✗ No" };
        let value_style = if value {
            colors.text_success()
        } else {
            colors.text_muted()
        };

        Line::from(vec![
            Span::styled(format!(" {} ", cursor), label_style),
            Span::styled(format!("{:<20}", label), label_style),
            Span::styled(value_display, value_style),
        ])
    };

    // Helper for optional bool fields (None/Some(true)/Some(false))
    let tri_state_line = |label: &str, value: Option<bool>, field: RuleEditorField, current: RuleEditorField| -> Line {
        let is_focused = field == current;
        let cursor = if is_focused { "▸" } else { " " };
        let label_style = if is_focused {
            colors.text_primary().add_modifier(Modifier::BOLD)
        } else {
            colors.text_dim()
        };
        let (value_display, value_style) = match value {
            None => ("─ Any", colors.text_muted()),
            Some(true) => ("✓ Yes", colors.text_success()),
            Some(false) => ("✗ No", colors.text_error()),
        };

        Line::from(vec![
            Span::styled(format!(" {} ", cursor), label_style),
            Span::styled(format!("{:<20}", label), label_style),
            Span::styled(value_display, value_style),
        ])
    };

    // Section: Basic Info
    lines.push(Line::from(vec![Span::styled(
        " ─── Basic ───",
        colors.text_primary().add_modifier(Modifier::BOLD),
    )]));
    lines.push(field_line("Name", &editor.name, RuleEditorField::Name, editor.field));
    lines.push(toggle_line("Enabled", editor.enabled, RuleEditorField::Enabled, editor.field));
    lines.push(Line::from(""));

    // Section: Conditions
    lines.push(Line::from(vec![Span::styled(
        " ─── Conditions ───",
        colors.text_primary().add_modifier(Modifier::BOLD),
    )]));
    lines.push(field_line("Extension", &editor.extension, RuleEditorField::Extension, editor.field));
    lines.push(field_line("Name Glob", &editor.name_glob, RuleEditorField::NameGlob, editor.field));
    lines.push(field_line("Name Regex", &editor.name_regex, RuleEditorField::NameRegex, editor.field));
    lines.push(field_line("Size > (bytes)", &editor.size_greater, RuleEditorField::SizeGreater, editor.field));
    lines.push(field_line("Size < (bytes)", &editor.size_less, RuleEditorField::SizeLess, editor.field));
    lines.push(field_line("Age > (days)", &editor.age_greater, RuleEditorField::AgeGreater, editor.field));
    lines.push(field_line("Age < (days)", &editor.age_less, RuleEditorField::AgeLess, editor.field));
    lines.push(tri_state_line("Is Directory", editor.is_directory, RuleEditorField::IsDirectory, editor.field));
    lines.push(tri_state_line("Is Hidden", editor.is_hidden, RuleEditorField::IsHidden, editor.field));
    lines.push(Line::from(""));

    // Section: Action
    lines.push(Line::from(vec![Span::styled(
        " ─── Action ───",
        colors.text_primary().add_modifier(Modifier::BOLD),
    )]));

    // Action type selector
    let is_focused = editor.field == RuleEditorField::ActionType;
    let cursor = if is_focused { "▸" } else { " " };
    let label_style = if is_focused {
        colors.text_primary().add_modifier(Modifier::BOLD)
    } else {
        colors.text_dim()
    };
    lines.push(Line::from(vec![
        Span::styled(format!(" {} ", cursor), label_style),
        Span::styled(format!("{:<20}", "Action Type"), label_style),
        Span::styled(format!("◀ {} ▶", editor.action_type.name()), colors.text_secondary()),
    ]));

    // Show relevant action fields based on action type
    match editor.action_type {
        ActionTypeSelection::Move | ActionTypeSelection::Copy => {
            lines.push(field_line("Destination", &editor.action_destination, RuleEditorField::ActionDestination, editor.field));
        }
        ActionTypeSelection::Rename => {
            lines.push(field_line("Pattern", &editor.action_pattern, RuleEditorField::ActionPattern, editor.field));
        }
        ActionTypeSelection::Run => {
            lines.push(field_line("Command", &editor.action_command, RuleEditorField::ActionCommand, editor.field));
        }
        ActionTypeSelection::Archive => {
            lines.push(field_line("Destination", &editor.action_destination, RuleEditorField::ActionDestination, editor.field));
        }
        ActionTypeSelection::Trash | ActionTypeSelection::Delete | ActionTypeSelection::Nothing => {
            // No additional fields needed
        }
    }

    let form = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(colors.primary))
                .border_type(BorderType::Rounded)
                .style(Style::default().bg(colors.bg))
                .title(title)
                .title_style(colors.text_primary())
                .title_bottom(Line::from(" Tab: next │ Shift+Tab: prev │ Enter: save │ Esc: cancel ").centered()),
        );

    frame.render_widget(form, popup_area);
}
