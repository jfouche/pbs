use std::{io, time::Duration};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyModifiers},
    execute, style, terminal,
};
use page::Page;
use screen::Screen;

mod page;
mod screen;
mod widget;

struct App<'a, W> {
    w: &'a mut W,
    page: Page,
    // store: Store,
}

impl<'a, W> App<'a, W>
where
    W: io::Write,
{
    pub fn new(w: &'a mut W) -> std::result::Result<Self, pbs_core::Error> {
        // let store = Store::open("store.db3")?;
        Ok(App {
            w,
            page: Page::home(),
            // store,
        })
    }

    pub fn run(&mut self) -> io::Result<()> {
        execute!(self.w, terminal::EnterAlternateScreen)?;

        terminal::enable_raw_mode()?;
        execute!(
            self.w,
            style::ResetColor,
            terminal::Clear(terminal::ClearType::All),
        )?;

        let (width, height) = terminal::size()?;
        let mut screen = Screen::new(width as usize, height as usize);

        loop {
            // Display page
            screen.add(&self.page);

            // Handle event
            if event::poll(Duration::from_millis(33))? {
                // It's guaranteed that the `read()` won't block when the `poll()`
                // function returns `true`
                match event::read()? {
                    Event::Key(ev)
                        if ev.code == KeyCode::Char('x')
                            && ev.modifiers.contains(KeyModifiers::CONTROL) =>
                    {
                        break
                    }
                    ev => self.page.handle_event(ev),
                }
            }

            screen.render(self.w)?;
        }

        // cleanup
        execute!(
            self.w,
            style::ResetColor,
            cursor::Show,
            terminal::LeaveAlternateScreen
        )?;

        terminal::disable_raw_mode()?;
        Ok(())
    }
}

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    match App::new(&mut stdout) {
        Ok(mut app) => app.run()?,
        Err(e) => {
            eprint!("ERROR : {e:?}");
        }
    }
    Ok(())
}
