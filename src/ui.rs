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
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),       // header area
            Constraint::Percentage(40),   // middle section (will be split horizontally)
            Constraint::Percentage(60),   // advanced details area
        ])
        .split(frame.size());
        let middle_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),   // cities list (left)
            Constraint::Percentage(50),   // basic weather (right)
        ])
        .split(main_chunks[1]);
        let header_area = main_chunks[0];
        let cities_area = middle_chunks[0];
        let basic_weather_area = middle_chunks[1];
        let advanced_details_area = main_chunks[2];
    // TODO: get the list of cities
    // let cities: Vec<ListItem> = 
    // let list_component =

    let static_text = Paragraph::new(Text::from("Welcome to CLI Weather App!"))
    .block(Block::default().borders(Borders::ALL))
    .style(Style::default().fg(Color::Blue))
    .wrap(Wrap { trim: true });
frame.render_widget(static_text, header_area);

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
    frame.render_widget(list_component, cities_area);
    // TODO: render the list of cities
    // frame.render_widget(list_component, area);
    let selected_city = app.cities.iter().find(|city| city.selected);

     if let Some(city) = selected_city {
        let weather_info = Paragraph::new(format!(
            "Weather for: {}\n\nTemp: {}°C\nHumidity: {}%\nWind: {} m/s",
            city.name, city.temperature, city.humidity, city.wind_speed
        ))
        .block(Block::default()
            .title("Basic Weather Details")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan)))
        .wrap(Wrap { trim: true });
    
        frame.render_widget(weather_info, basic_weather_area);
    } else {
        let no_selection = Paragraph::new("No city selected")
            .block(Block::default().title("Weather Details").borders(Borders::ALL))
            .wrap(Wrap { trim: true });
        frame.render_widget(no_selection, basic_weather_area);
    }

    // TODO: Create the weather info component
    // let weather_info =
    if let Some(city) = selected_city {
        // Create advanced weather details paragraph
        let advanced_info = Paragraph::new(format!(
            "Feels Like: {}°C\nMin/Max: {}°C / {}°C\nPressure: {} hPa\n\nWeather: {}\nDescription: {}\n\nWind Direction: {}°\nCloudiness: {}%\nVisibility: {} m\n\nSunrise: {}\nSunset: {}",
            city.feels_like,
            city.temp_min,
            city.temp_max,
            city.pressure,
            city.weather_main,
            city.weather_description,
            city.wind_direction,
            city.cloudiness,
            city.visibility,
            city.sunrise_formatted(),
            city.sunset_formatted()
        ))
        .block(Block::default()
            .title("Advanced Weather Details")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan)))
        .wrap(Wrap { trim: true });
    
        // Render the advanced weather info component
        frame.render_widget(advanced_info, advanced_details_area);
    } else {
        // Fallback if no city is selected
        let no_selection = Paragraph::new("No city selected")
            .block(Block::default()
                .title("Advanced Weather Details")
                .borders(Borders::ALL))
            .wrap(Wrap { trim: true });
        
        frame.render_widget(no_selection, advanced_details_area);
    }
    // TODO: Render the weather info component
    // frame.render_widget(weather_info, area);

    
}
