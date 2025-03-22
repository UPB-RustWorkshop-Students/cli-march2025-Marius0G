use ratatui::{
    Frame,
    layout::{Layout, Direction, Constraint, Rect},
    style::{Style, Color},
    widgets::{Block, Borders, Paragraph, Wrap, List, ListItem, ListState},
    text::{Text, Span, Line},

};
use crate::app::App;


/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui/ratatui/tree/main/ratatui/examples

    
    // TODO: Split the layout
    // let [area1, area2, area3 ...] =
    let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
        Constraint::Length(3),  // area1
        Constraint::Percentage(40),  // area2
        Constraint::Percentage(40),  // area3
    ])
    .split(frame.size());
    let [area1, area2, area3] = [chunks[0], chunks[1], chunks[2]];


    // TODO: get the list of cities
    // let cities: Vec<ListItem> = 
    // let list_component =

    let static_text = Paragraph::new(Text::from("Welcome to CLI Weather App!"))
    .block(Block::default().borders(Borders::ALL))
    .style(Style::default().fg(Color::Blue))
    .wrap(Wrap { trim: true });

    frame.render_widget(static_text,area1);

    // Get the list of cities
    let cities: Vec<ListItem> = app.cities
    .iter()
    .enumerate()
    .map(|(index, city)| {
        // Create marker based on selected status
        let prefix = if city.selected { "[X] " } else { "[ ] " };
        
        // Create the ListItem with proper styling
        ListItem::new(Line::from(vec![
            Span::styled(prefix, Style::default().fg(Color::Yellow)),
            Span::raw(&city.name),
        ]))
    })
    .collect();

    // Create a block for the cities list
    let cities_block = Block::default()
    .title("Cities")
    .borders(Borders::ALL)
    .border_style(Style::default().fg(Color::Cyan));

    // Create the List component
    let list_component = List::new(cities)
    .block(cities_block)
    .highlight_style(Style::default().fg(Color::Yellow))
    .highlight_symbol("> ");
    // Render the list of cities
    frame.render_widget(list_component, area2);
    // TODO: render the list of cities
    // frame.render_widget(list_component, area);
    let selected_city = app.cities.iter().find(|city| city.selected);

    if let Some(city) = selected_city {
        // Create a weather info paragraph with the city's weather data
        let weather_info = Paragraph::new(format!(
            "Weather for: {}\n\nTemp: {}°C\nHumidity: {}%\nWind: {} m/s",
            city.name, city.temperature, city.humidity, city.wind_speed
        ))
        .block(Block::default()
            .title("Weather Details")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan)))
        .wrap(Wrap { trim: true });
    
        // Render the weather info component
        frame.render_widget(weather_info, area3);
    } else {
        // Fallback if no city is selected
        let no_selection = Paragraph::new("No city selected")
            .block(Block::default()
                .title("Weather Details")
                .borders(Borders::ALL))
            .wrap(Wrap { trim: true });
        
        frame.render_widget(no_selection, area3);
    }

    // TODO: Create the weather info component
    // let weather_info =

    // TODO: Render the weather info component
    // frame.render_widget(weather_info, area);

    
}
