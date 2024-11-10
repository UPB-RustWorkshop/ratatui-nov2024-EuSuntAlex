use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Modifier, Style};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};
use ratatui::{Frame, widgets::ListState};
use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(frame.area());

    // Inițializăm ListState pentru a gestiona scrolling-ul și selecția
    let mut list_state = ListState::default();
    list_state.select(Some(app.indicator));

    // Setăm numărul de elemente vizibile în listă 
    let num_visible_items = 14;
    let start_index = app.indicator.saturating_sub(num_visible_items / 2); 
    let end_index = (start_index + num_visible_items).min(app.city.len());
    
    // Filtrăm lista pentru a afișa doar fereastra de 10 elemente
    let visible_items: Vec<ListItem> = app
        .city[start_index..end_index]
        .iter()
        .enumerate()
        .map(|(i, &city)| {
            let mut item = ListItem::new(city);
            // Aplicăm stilul de evidențiere doar pentru elementul selectat
            if start_index + i == app.indicator {
                item = item.style(Style::default().add_modifier(Modifier::BOLD));
            }
            item
        })
        .collect();
    
    // Creăm lista de orașe în coloana din stânga, cu evidențiere și scrolling
    let list_component = List::new(visible_items)
        .block(
            Block::default()
                .title("City")
                .borders(Borders::ALL)
        )
        .highlight_symbol("> ");

    frame.render_stateful_widget(list_component, chunks[0], &mut list_state);

    // Împărțim coloana din dreapta în două rânduri pentru liniile de text
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[1]);

    // Afișăm orașul selectat în prima secțiune din dreapta
    let first_line = Paragraph::new(app.city[app.indicator])
        .block(
            Block::default()
                .title("Info")
                .borders(Borders::ALL)
        );
    frame.render_widget(first_line, right_chunks[0]);

    // Dacă avem date despre vreme, le afișăm, altfel afișăm un mesaj de eroare
    if let Some(data) = &app.data {
        let details_text = format!("Temperature: {:.1}°C\nHumidity: {:.1}%", data.temp, data.hum);
        let second_line = Paragraph::new(details_text)
            .block(
                Block::default()
                    .title("Detalii")
                    .borders(Borders::ALL)
            );
        frame.render_widget(second_line, right_chunks[1]);

        // Determinăm mesajul în funcție de temperatura curentă
        let message = if data.temp < 15.0 {
            "It will be cold"
        } else if data.temp > 30.0 {
            "It will be hot"
        } else {
            "The weather is moderate"
        };
        
        let third_line = Paragraph::new(message)
            .block(
                Block::default()
                    .title("Forecast")
                    .borders(Borders::ALL)
            );
        frame.render_widget(third_line, right_chunks[1]); 
    } else {
        let second_line = Paragraph::new("Could not load info :(")
            .block(
                Block::default()
                    .title("Detalii")
                    .borders(Borders::ALL)
            ); 
        frame.render_widget(second_line, right_chunks[1]);
    }
}

