use std::time;

use crossterm::event::{self, Event, KeyCode};
use ratatui::prelude::*;
use ratatui::widgets::Widget;

pub struct Timer {
    start_time: time::Instant,
}

impl Timer {
    pub fn new() -> Self {
        let initial_time = time::Instant::now();

        Timer {
            start_time: time::Instant::now(),
        }
    }

    pub fn handle_events(&mut self) -> std::io::Result<()> {
        loop {
            if event::poll(time::Duration::from_millis(100))
                .expect("Crossterm error, run 'cargo fetch'")
            {
                if let Event::Key(key) = event::read().expect("Crossterm error, run 'cargo fetch'")
                {
                    if key.code == KeyCode::Char('s') {
                        println!("\n\rTimer stopped.");
                        println!("");
                        break;
                    } else {
                        continue;
                    }
                }
            }
        }
        Ok(())
    }
}

impl Widget for Timer {
    fn render(self, area: ratatui::layout::Rect, buf: &mut ratatui::buffer::Buffer) {
        // Implement rendering logic for the timer here
    }
}
