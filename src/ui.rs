use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
    Frame,
};

use crate::app::{App, BodyType};

pub fn draw(f: &mut Frame, app: &App) {
    let area = f.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(area);

    // Header
    let header_text = Line::from(vec![
        Span::styled(" My Sky  ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw("│  Kingston, ON  44.2°N  76.5°W  │  "),
        Span::styled(app.header_time(), Style::default().fg(Color::Yellow)),
        Span::raw("  │  [u] UTC/Local  [q] Quit"),
    ]);
    let header = Paragraph::new(header_text)
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::DarkGray)));
    f.render_widget(header, chunks[0]);

    // Table
    let header_cells = ["Body", "Alt", "Az", "Rise", "Transit", "Set", "Mag"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD)));
    let table_header = Row::new(header_cells)
        .style(Style::default().bg(Color::DarkGray))
        .height(1);

    let rows: Vec<Row> = app.bodies.iter().map(|body| {
        let style = body_style(body.alt, &body.body_type);

        let alt_str = fmt_angle(body.alt);
        let az_str  = fmt_angle(body.az);

        let rise_str = if body.never_rises {
            "—".to_string()
        } else if body.circumpolar {
            "∞".to_string()
        } else {
            body.rise_min.map(|m| app.fmt_time(m)).unwrap_or_default()
        };

        let transit_str = if body.never_rises {
            "—".to_string()
        } else {
            body.transit_min.map(|m| app.fmt_time(m)).unwrap_or_default()
        };

        let set_str = if body.never_rises {
            "—".to_string()
        } else if body.circumpolar {
            "∞".to_string()
        } else {
            body.set_min.map(|m| app.fmt_time(m)).unwrap_or_default()
        };

        let mag_str = body.mag
            .map(|m| format!("{:+.1}", m))
            .unwrap_or_default();

        let name_style = match body.body_type {
            BodyType::Sun    => style.add_modifier(Modifier::BOLD),
            BodyType::Moon   => style.add_modifier(Modifier::BOLD),
            BodyType::Planet => style.add_modifier(Modifier::BOLD),
            BodyType::Star   => style,
        };

        Row::new(vec![
            Cell::from(body.name.clone()).style(name_style),
            Cell::from(alt_str).style(style),
            Cell::from(az_str).style(style),
            Cell::from(rise_str).style(Style::default().fg(Color::DarkGray)),
            Cell::from(transit_str).style(Style::default().fg(Color::DarkGray)),
            Cell::from(set_str).style(Style::default().fg(Color::DarkGray)),
            Cell::from(mag_str).style(Style::default().fg(Color::DarkGray)),
        ])
        .height(1)
    }).collect();

    let table = Table::new(
        rows,
        [
            Constraint::Length(16),
            Constraint::Length(8),
            Constraint::Length(8),
            Constraint::Length(9),
            Constraint::Length(9),
            Constraint::Length(9),
            Constraint::Length(5),
        ],
    )
    .header(table_header)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray))
            .title(" Celestial Bodies "),
    )
    .column_spacing(1);

    f.render_widget(table, chunks[1]);
}

fn body_style(alt: f64, body_type: &BodyType) -> Style {
    match body_type {
        BodyType::Sun => {
            if alt > 0.0 {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default().fg(Color::Rgb(80, 80, 0))
            }
        }
        BodyType::Moon => {
            if alt > 0.0 {
                Style::default().fg(Color::Rgb(200, 200, 255))
            } else {
                Style::default().fg(Color::Rgb(60, 60, 80))
            }
        }
        BodyType::Planet => {
            if alt > 15.0 {
                Style::default().fg(Color::Green)
            } else if alt > 0.0 {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default().fg(Color::Rgb(40, 60, 40))
            }
        }
        BodyType::Star => {
            if alt > 15.0 {
                Style::default().fg(Color::White)
            } else if alt > 0.0 {
                Style::default().fg(Color::Rgb(180, 180, 100))
            } else {
                Style::default().fg(Color::Rgb(60, 60, 60))
            }
        }
    }
}

fn fmt_angle(deg: f64) -> String {
    let sign = if deg < 0.0 { "-" } else { " " };
    let abs = deg.abs();
    let d = abs.floor() as u32;
    let m = ((abs - d as f64) * 60.0).round() as u32;
    format!("{}{:3}°{:02}′", sign, d, m)
}
