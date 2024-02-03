use std::{io, time::Duration};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyModifiers},
    execute, style, terminal,
};
use screen::Screen;

mod screen;

enum Page {
    Search(PageSeach),
}

impl Page {
    fn home() -> Self {
        Page::Search(PageSeach::new())
    }

    fn display(&mut self, w: &mut Screen) {
        match self {
            Page::Search(page) => page.display(w),
        }
    }

    fn handle_event(&mut self, event: Event) {
        match self {
            Page::Search(page) => page.handle_event(event),
        }
    }
}

struct PageSeach {
    pattern: String,
}

impl PageSeach {
    fn new() -> Self {
        PageSeach {
            pattern: "".to_string(),
        }
    }

    fn display(&mut self, screen: &mut Screen) {
        screen.display_title("SEARCH");
        screen.put_str("Pattern", 1, 3);
    }

    fn handle_event(&mut self, event: Event) {}
}

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
        // execute!(self.w, terminal::EnterAlternateScreen)?;

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
            self.page.display(&mut screen);

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
            // terminal::LeaveAlternateScreen
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
