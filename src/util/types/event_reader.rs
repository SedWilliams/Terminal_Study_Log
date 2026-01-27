/*********************************
 * EventReader Types
 *********************************/

use crossterm::event;

use super::EventResult;

pub trait EventReader {
    fn read_event(&mut self) -> EventResult;
    fn poll_event(&mut self) -> EventResult;
}

pub struct TerminalEventReader;

impl TerminalEventReader {
    pub fn new() -> Self {
        TerminalEventReader
    }
}

impl EventReader for TerminalEventReader {
    fn read_event(&self) -> EventResult {
        Ok(crossterm::event::read()?)
    }

    fn poll_event(&self) -> eventresult {
        loop {
            if poll(Duration::from_millis(1000))? {
                if let event::key(key) = read()? {
                    return Ok(event::key());
                }
            }
        }
    }
}
