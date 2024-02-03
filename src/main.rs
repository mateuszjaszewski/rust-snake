use std::io;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;
use crossterm::{cursor, ExecutableCommand, QueueableCommand, style, terminal};
use crossterm::event::{self, Event};
use crossterm::style::Stylize;

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    let mut x = 0;
    let mut y = 0;

    loop {
        stdout
            .execute(terminal::Clear(terminal::ClearType::All))?
            .execute(cursor::MoveTo(x, y))?
            .execute(style::PrintStyledContent("O".green()))?
            .flush()?;
        x += 1;
        y += 1;

        if x > 20 { x = 0; }
        if y > 20 { y = 0; }

        sleep(Duration::from_millis(200));

    /*
        match event::read()? {
            Event::Key(event) =>  {
                println!("{:?}", event);
                return Ok(())
            }
            _ => {}
        }
    */
    }
}
