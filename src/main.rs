use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
};
use crossterm::{terminal, ExecutableCommand};
use invaders::{frame::{self, new_frame}, render};
use rusty_audio::Audio;
use std::io;
use std::{
    error::Error,
    sync::mpsc,
    thread::{self},
    time::Duration,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add("explode", "./assets/explode.wav");
    audio.add("lose", "./assets/lose.wav");
    audio.add("move", "./assets/move.wav");
    audio.add("pew", "./assets/pew.wav");
    audio.add("startup", "./assets/startup.wav");
    audio.add("win", "./assets/win.wav");
    audio.play("startup");

    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // Render loop in a separate thread
    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);
        loop {
            let curr_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };
            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame
        }
    });

    // Game loop
    'gameloop: loop {
        // Per frame init
        let curr_frame = new_frame();

        // Input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop;
                    }
                    _ => {}
                }
            }
        }

        // Draw and Render Section
        let _ = render_tx.send(curr_frame); // Silently ignoring the error
        thread::sleep(Duration::from_millis(1));
    }

    // Clean up
    drop(render_tx); // doesn't need in newer rusts
    render_handle.join().unwrap();

    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
