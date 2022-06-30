use std::error::Error;
use std::{io, thread};
use std::sync::mpsc;
use std::time::Duration;
use crossterm::{event, ExecutableCommand, terminal};
use crossterm::cursor::{Hide, Show};
use crossterm::event::{Event, KeyCode};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use rusty_audio::Audio;
use console_invaders::{AUDIO_DIRECTORY, AUDIO_EXPLODE, AUDIO_FILE_EXTENSION, AUDIO_LOSE, AUDIO_MOVE, AUDIO_PEW, AUDIO_STARTUP, AUDIO_WIN, frame, render};
use console_invaders::frame::new_frame;


fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add(AUDIO_EXPLODE, AUDIO_DIRECTORY.to_owned() + AUDIO_EXPLODE + AUDIO_FILE_EXTENSION);
    audio.add(AUDIO_LOSE, AUDIO_DIRECTORY.to_owned() + AUDIO_LOSE + AUDIO_FILE_EXTENSION);
    audio.add(AUDIO_MOVE, AUDIO_DIRECTORY.to_owned() + AUDIO_MOVE + AUDIO_FILE_EXTENSION);
    audio.add(AUDIO_PEW, AUDIO_DIRECTORY.to_owned() + AUDIO_EXPLODE + AUDIO_FILE_EXTENSION);
    audio.add(AUDIO_STARTUP, AUDIO_DIRECTORY.to_owned() + AUDIO_STARTUP + AUDIO_FILE_EXTENSION);
    audio.add(AUDIO_WIN, AUDIO_DIRECTORY.to_owned() + AUDIO_WIN + AUDIO_FILE_EXTENSION);
    audio.play(AUDIO_STARTUP);

    // start terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?; // enable keyboard input
    stdout.execute(EnterAlternateScreen)?; // switch to an alternate screen (like Vim does)
    stdout.execute(Hide)?; // hide cursor

    // render loop in a separate thread
    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        // force render entire frame once
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);

        // render frame deltas
        loop {
            let current_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };
            render::render(&mut stdout, &last_frame, &current_frame, false);
            last_frame = current_frame;
        }
    });

    // game loop
    'gameloop: loop {
        // per-frame initialization
        let current_frame = new_frame();


        // input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play(AUDIO_LOSE);
                        break 'gameloop;
                    }
                    _ => {}
                }
            }
        }

        // draw and render
        let _ = render_tx.send(current_frame); // ignore the error because game loop won't be fully set up initially
        thread::sleep(Duration::from_millis(1));
    }

    // clean up
    render_handle.join().unwrap();
    audio.wait();
    stdout.execute(Show)?; // show cursor
    stdout.execute(LeaveAlternateScreen)?; // leave alternate screen
    terminal::disable_raw_mode()?; // disable keyboard input
    Ok(())
}
