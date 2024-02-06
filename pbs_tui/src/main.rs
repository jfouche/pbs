mod page;
mod screen;
mod widget;

use std::{io, time::Duration};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyModifiers},
    execute, style, terminal,
};
use page::Page;
use screen::Screen;
use widget::{Prompt, Widget};

struct MainWindow {
    page: Page,
    prompt: Prompt,
}

impl MainWindow {
    fn new() -> Self {
        MainWindow {
            page: Page::home(),
            prompt: Prompt::default(),
        }
    }
}

impl Widget for MainWindow {
    fn display(&self, buf: &mut widget::Buffer) {
        self.page.display(buf);
        self.prompt.display(buf);
    }

    fn handle_event(&mut self, event: &Event) {
        match self.page {
            Page::Help(_) => self.prompt.set_label("> "),
            Page::Search(_) => self.prompt.set_label("search> "),
        }

        self.page.handle_event(event);
        self.prompt.handle_event(event);
    }
}

struct App<'a, W> {
    w: &'a mut W,
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
            // store,
        })
    }

    fn run_loop(&mut self, mut screen: Screen) -> io::Result<()> {
        let mut wnd = MainWindow::new();
        loop {
            // Display page
            screen.add(&mut wnd);

            // Handle event
            if event::poll(Duration::from_millis(33))? {
                // It's guaranteed that the `read()` won't block when the `poll()`
                // function returns `true`
                match event::read()? {
                    Event::Key(ev)
                        if ev.code == KeyCode::Char('x')
                            && ev.modifiers.contains(KeyModifiers::CONTROL) =>
                    {
                        // Exit program
                        return Ok(());
                    }
                    ev => wnd.handle_event(&ev),
                }
            }
            screen.render(self.w)?;
        }
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
        let screen = Screen::new(width as usize, height as usize);
        self.run_loop(screen)?;
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
