use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event as CEvent, KeyCode};
use std::sync::mpsc;
use std::time::{Duration, Instant};
use std::{io, thread};

use crate::key::Key;
//use crate::key::Key;

pub enum Event<I> {
    Input(I),
    Tick,
}
pub(crate) struct Events {
    //rx: mpsc::Receiver<Event<event::KeyCode>>,
    rx: mpsc::Receiver<Event<Key>>,
}

impl Events {
    pub fn new() -> Events {
        let (tx, rx) = mpsc::channel();

        let tx_clone = tx.clone();
        let tick_rate = Duration::from_millis(200);

        thread::spawn(move || {
            let mut last_tick = Instant::now();
            loop {
                let timeout = tick_rate
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or_else(|| Duration::from_secs(0));

                if event::poll(timeout).expect("poll works") {
                    if let CEvent::Key(key) = event::read().expect("can read events") {
                        let rip = Key::from(key);
                        tx_clone.send(Event::Input(rip)).expect("can send events");
                    }
                }

                if last_tick.elapsed() >= tick_rate {
                    if let Ok(_) = tx_clone.send(Event::Tick) {
                        last_tick = Instant::now();
                    }
                }
            }
        });

        Events { rx }
    }

    pub fn next(&self) -> Result<Event<Key>, mpsc::RecvError> {
        self.rx.recv()
    }
}