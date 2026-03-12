use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
    Frame, Terminal,
};
use std::io::{self, Stdout};

#[derive(Clone, PartialEq)]
pub enum SimPhase {
    Random,
    Exiting,
}

#[derive(Clone)]
pub struct SimStats {
    pub people_remaining: usize,
    pub total_people: usize,
    pub total_doors: usize,
    pub room_size: usize,
    pub elapsed_secs: u64,
    pub phase: SimPhase,
}

pub fn setup_terminal() -> io::Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    Terminal::new(backend)
}

pub fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> io::Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()
}

pub fn render(frame: &mut Frame, matrix: &[Vec<i32>], stats: &SimStats) {
    let outer = Block::default()
        .borders(Borders::ALL)
        .title(" Room Simulation ");
    let inner_area = outer.inner(frame.area());
    frame.render_widget(outer, frame.area());

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(0), Constraint::Length(24)])
        .split(inner_area);

    // --- Grid (left panel) ---
    let rows: Vec<Row> = matrix
        .iter()
        .map(|row| {
            let cells: Vec<Cell> = row
                .iter()
                .map(|&val| match val {
                    0 => Cell::from(Span::styled(
                        "· ",
                        Style::default().fg(Color::DarkGray),
                    )),
                    -1 => Cell::from(Span::styled(
                        "▣ ",
                        Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
                    )),
                    n => {
                        const PALETTE: &[Color] = &[
                            Color::Cyan,
                            Color::Yellow,
                            Color::Magenta,
                            Color::Red,
                            Color::Blue,
                            Color::LightCyan,
                            Color::LightYellow,
                            Color::LightMagenta,
                            Color::LightRed,
                            Color::LightBlue,
                            Color::LightGreen,
                            Color::White,
                        ];
                        let color = PALETTE[(n as usize).wrapping_sub(1) % PALETTE.len()];
                        Cell::from(Span::styled(
                            "● ",
                            Style::default().fg(color).add_modifier(Modifier::BOLD),
                        ))
                    }
                })
                .collect();
            Row::new(cells)
        })
        .collect();

    let col_widths: Vec<Constraint> = (0..stats.room_size)
        .map(|_| Constraint::Length(2))
        .collect();

    let grid = Table::new(rows, col_widths)
        .block(Block::default().borders(Borders::RIGHT));
    frame.render_widget(grid, chunks[0]);

    // --- Stats sidebar (right panel) ---
    let exited = stats.total_people - stats.people_remaining;
    let phase_style = match stats.phase {
        SimPhase::Random => Style::default().fg(Color::Yellow),
        SimPhase::Exiting => Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
    };
    let phase_label = match stats.phase {
        SimPhase::Random => "Random",
        SimPhase::Exiting => "Exiting",
    };

    let text = Text::from(vec![
        Line::from(Span::styled(
            "ROOM SIMULATION",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from("────────────────"),
        Line::from(vec![
            Span::raw("People:  "),
            Span::styled(
                format!("{} / {}", stats.people_remaining, stats.total_people),
                Style::default().fg(Color::Cyan),
            ),
        ]),
        Line::from(vec![
            Span::raw("Exited:  "),
            Span::styled(
                format!("{}", exited),
                Style::default().fg(Color::Green),
            ),
        ]),
        Line::from(vec![
            Span::raw("Doors:   "),
            Span::styled(
                format!("{}", stats.total_doors),
                Style::default().fg(Color::Green),
            ),
        ]),
        Line::from(vec![
            Span::raw("Size:    "),
            Span::styled(
                format!("{}×{}", stats.room_size, stats.room_size),
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::raw("Phase:   "),
            Span::styled(phase_label, phase_style),
        ]),
        Line::from(vec![
            Span::raw("Time:    "),
            Span::styled(
                format!("{}s", stats.elapsed_secs),
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from("────────────────"),
        Line::from(Span::styled(
            "[q]  quit",
            Style::default().fg(Color::DarkGray),
        )),
    ]);

    let sidebar = Paragraph::new(text)
        .block(Block::default().borders(Borders::NONE));
    frame.render_widget(sidebar, chunks[1]);
}
