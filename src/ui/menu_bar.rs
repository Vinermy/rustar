use tui::style::{Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Tabs};
use crate::style::*;
use crate::menu_item::MenuItem;


pub fn render_menu_bar<'a>(
    menu_titles: &'a Vec<&'a str>,
    active_item: &'a MenuItem)
        -> Tabs<'a> {

    let menu = menu_titles
        .iter()
        .map(|t| {
            let (first, rest) = t.split_at(1);
            Spans::from(vec![
                Span::styled(
                    first,
                    Style::default()
                        .fg(FOCUS_COLOR)
                        .add_modifier(Modifier::UNDERLINED),
                ),
                Span::styled(rest, Style::default().fg(ACTIVE_COLOR)),
            ])
        })
        .collect();

    let tabs = Tabs::new(menu)
        .select(active_item.into())
        .block(Block::default().title("Menu").borders(Borders::ALL))
        .style(Style::default().fg(ACTIVE_COLOR))
        .highlight_style(Style::default().fg(FOCUS_COLOR))
        .divider(Span::raw("|"));

    tabs
}
