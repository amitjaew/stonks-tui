use ratatui::{
    prelude::*,
    widgets::*
};
//use chrono::{ DateTime, Utc };
use crate::utils::api::{
    get_btc_usd_comp,
    get_clp_usd_comp,
    get_xmr_usd_comp
};


struct GraphBounds {
    bounds_x: [f64; 2],
    bounds_y: [f64; 2],
}

fn get_bounds(points: Vec<(f64, f64)>) -> GraphBounds {
    let bounds_x = [
        points.iter()
            .map(|&(x, _)| x)
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap(),
        points.iter()
            .map(|&(x, _)| x)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap()
    ];
    let bounds_y = [
        ( 100.0 * points.iter()
                        .map(|&(_, y)| y)
                        .min_by(|a, b| a.partial_cmp(b).unwrap())
                        .unwrap()
        ).floor() / 100.0,
        ( 100.0 * points.iter()
                        .map(|&(_, y)| y)
                        .max_by(|a, b| a.partial_cmp(b).unwrap())
                        .unwrap()
        ).ceil() / 100.0
    ];

    GraphBounds {
        bounds_x,
        bounds_y
    }
}

fn get_chart(
    datasets: Vec<Dataset>,
    title: String,
    x_title: String,
    y_title: String,
    date_labels: Vec<String>,
    bounds: GraphBounds
    //base_date: DateTime<Utc>
) -> Chart<'_> {
    let bounds_x = bounds.bounds_x;
    let bounds_y = bounds.bounds_y;
    let mut x_labels: Vec<Span> = Vec::new();
    for l in date_labels {
        x_labels.push(l.to_owned().into());
    }

    let x_axis = Axis::default()
        .title(x_title.red())
        .style(Style::default().blue())
        .bounds(bounds_x)
        .labels(x_labels);
    
    let y_axis = Axis::default()
        .title(y_title.red())
        .style(Style::default().blue())
        .bounds(bounds_y)
        .labels(vec![
            bounds_y[0].round().to_string().into(),
            (bounds_y[0].round() + ((bounds_y[1] - bounds_y[0]) * 0.34).round()).to_string().into(),
            (bounds_y[0].round() + ((bounds_y[1] - bounds_y[0]) * 0.67).round()).to_string().into(),
            bounds_y[1].round().to_string().into()
        ]);

    Chart::new(datasets)
        .block(Block::default().title(title).light_blue().bold())
        .x_axis(x_axis)
        .y_axis(y_axis)
}


pub async fn gui_startup() -> Result<(), Box<dyn std::error::Error>> {
    let data_xmr = get_xmr_usd_comp().await.unwrap();
    let points_xmr = data_xmr.get_points();
    let dates_xmr = data_xmr.get_date_labels();
    let bounds_xmr = get_bounds(points_xmr.clone());
    let chart_xmr = get_chart(
        vec![
            Dataset::default()
                .marker(symbols::Marker::Braille)
                .graph_type(GraphType::Line)
                .style(Style::default().cyan())
                .data(&points_xmr)
        ],
        String::from("XMR"),
        String::from("Date"),
        String::from("USD"),
        dates_xmr,
        bounds_xmr
    );

    let data_btc = get_btc_usd_comp().await.unwrap();
    let dates_btc = data_btc.get_date_labels();
    let points_btc = data_btc.get_points();
    let bounds_btc = get_bounds(points_btc.clone());
    let chart_btc = get_chart(
        vec![
            Dataset::default()
                .marker(symbols::Marker::Braille)
                .graph_type(GraphType::Line)
                .style(Style::default().cyan())
                .data(&points_btc)
        ],
        String::from("BTC"),
        String::from("Date"),
        String::from("USD"),
        dates_btc,
        bounds_btc
    );

    let data_clp = get_clp_usd_comp().await.unwrap();
    let dates_clp = data_clp.get_date_labels();
    let points_clp = data_clp.get_points();
    let bounds_clp = get_bounds(points_clp.clone());
    let chart_clp = get_chart(
        vec![
            Dataset::default()
                .marker(symbols::Marker::Braille)
                .graph_type(GraphType::Line)
                .style(Style::default().cyan())
                .data(&points_clp)
        ],
        String::from("USD"),
        String::from("Date"),
        String::from("CLP"),
        dates_clp,
        bounds_clp
    );

    // startup: Enable raw mode for the terminal, giving us fine control over user input
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen)?;

    // Initialize the terminal backend using crossterm
    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

    // Define our counter variable
    // This is the state of our application
    let mut counter = 0;

    // Main application loop
    loop {
        // Render the UI
        terminal.draw(|f| {
            f.render_widget(Paragraph::new(format!("Counter: {counter}")), f.size());

            let base = f.size();
            let padding_x = 1;
            let margin_x = 1;
            let padding_y = 1;
            let margin_y = 1;
            f.render_widget(&chart_xmr, Rect{
                x: base.width / 2 + padding_x + margin_x,
                y: margin_y,
                height: base.height / 2 - padding_y - margin_y,
                width: base.width / 2 - padding_x - margin_x
            });
            f.render_widget(&chart_clp, Rect{
                x: margin_x,
                y: base.height / 2 + padding_y + margin_y,
                height: base.height / 2 - padding_y - margin_y,
                width: base.width / 2 - padding_x - margin_x
            });
            f.render_widget(&chart_btc, Rect{
                x: base.width / 2 + padding_x + margin_x,
                y: base.height / 2 + padding_y + margin_y,
                height: base.height / 2 - padding_y - margin_y,
                width: base.width / 2 - padding_x - margin_x
            });
        })?;

        // Check for user input every 250 milliseconds
        if crossterm::event::poll(std::time::Duration::from_millis(250))? {
            // If a key event occurs, handle it
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                if key.kind == crossterm::event::KeyEventKind::Press {
                    match key.code {
                        crossterm::event::KeyCode::Char('j') => counter += 1,
                        crossterm::event::KeyCode::Char('k') => counter -= 1,
                        crossterm::event::KeyCode::Char('q') => break,
                        _ => {},
                    }
                }
            }
        }
    }

    // shutdown down: reset terminal back to original state
    crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}

