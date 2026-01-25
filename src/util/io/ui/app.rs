use crossterm::event::{self, Event, KeyCode};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Widget};
use ratatui::{self, DefaultTerminal, Frame, layout::Constraint};
use tui_big_text::{BigText, PixelSize};

#[derive(Default)]
pub struct App {
    exit: bool,
}

impl App {
    //takes in terminal and then passes the terminal frame to the functions below
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }

        Ok(())
    }

    //accepts a terminal frame to draw to, then draws to it
    //      called from run() above
    pub fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    //called to take care of the event handling
    //      we will implement this with crossterm
    pub fn handle_events(&mut self) -> std::io::Result<()> {
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(keypress) = event::read()? {
                match keypress.code {
                    KeyCode::Char('q') => self.exit = true,
                    _ => {}
                }
            }
        }
        Ok(())
    }
}

//since this is a top level component/app we dont need to impl widget
//     we can just put this logic in App::draw()
//
// For smaller components, we can impl Widget for them and then call them from App::draw()
impl Widget for &App {
    //this method is called by the render_widget method call above in self.draw()
    fn render(self, area: ratatui::layout::Rect, buf: &mut ratatui::buffer::Buffer) {
        let term_area = Rect::new(0, 0, area.width, area.height);

        let layout = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                Constraint::Min(10),
                Constraint::Percentage(30),
                Constraint::Min(10),
                Constraint::Percentage(50),
            ])
            .spacing(0)
            .margin(0)
            .split(term_area);

        let header = BigText::builder()
            .pixel_size(PixelSize::HalfHeight)
            .style(Style::new().white().bold())
            .lines(vec!["STUI Timer".into()])
            .centered()
            .build();

        let commands = Line::from(vec![
            " Start<s> ".into(),
            "Quit<q> ".into(),
            "Logs<v> ".into(),
        ]);

        let terminal_outline = Block::bordered()
            .blue()
            .bold()
            .title_bottom(commands.centered());

        header.render(layout[1], buf);
        terminal_outline.render(area, buf);
    }
}
