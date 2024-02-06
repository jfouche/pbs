mod page;
mod screen;
mod widget;

use std::{
    io,
    sync::{
        mpsc::{self, Sender},
        Arc,
    },
    thread,
    time::Duration,
};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyModifiers},
    execute, style, terminal,
};
use page::Page;
use pbs_core::{Item, Store};
use screen::Screen;
use widget::{Prompt, StatusBar, Widget};

pub enum PbsAction {
    Search(String),
}

enum PbsResponse {
    Err(String),
    Items(Vec<Item>),
}

struct MainWindow {
    page: Page,
    status: StatusBar,
    prompt: Prompt,
}

impl MainWindow {
    fn new() -> Self {
        MainWindow {
            page: Page::home(),
            status: StatusBar::default(),
            prompt: Prompt::default(),
        }
    }

    fn handle_response(&mut self, response: PbsResponse) {
        match (response, &mut self.page) {
            (PbsResponse::Err(err), _) => {
                self.status.text = err;
            }
            (PbsResponse::Items(items), Page::Search(ref mut page)) => {
                page.set_items(items);
            }
            _ => {}
        }
    }
}

impl Widget for MainWindow {
    type Action = PbsAction;

    fn display(&self, buf: &mut widget::Buffer) {
        self.page.display(buf);
        self.status.display(buf);
        self.prompt.display(buf);
    }

    fn handle_event(&mut self, event: &Event) -> Option<Self::Action> {
        match self.page {
            Page::Help(_) => self.prompt.set_label("> "),
            Page::Search(_) => self.prompt.set_label("search> "),
        }

        self.page.handle_event(event);
        if let Some(text) = self.prompt.handle_event(event) {
            match self.page {
                Page::Search(_) => {
                    if text.len() > 2 {
                        return Some(PbsAction::Search(text));
                    }
                }
                _ => {}
            }
        }
        None
    }
}

struct App<'a, W> {
    w: &'a mut W,
    store: Arc<Store>,
}

impl<'a, W> App<'a, W>
where
    W: io::Write,
{
    pub fn new(w: &'a mut W) -> std::result::Result<Self, pbs_core::Error> {
        let store = Store::open("store.db3")?;
        Ok(App {
            w,
            store: Arc::new(store),
        })
    }

    fn run_loop(&mut self, mut screen: Screen) -> io::Result<()> {
        let mut wnd = MainWindow::new();
        let (tx, rx) = mpsc::channel();
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
                    ev => {
                        if let Some(action) = wnd.handle_event(&ev) {
                            self.handle_action(action, tx.clone());
                        }
                    }
                }
            }

            if let Ok(response) = rx.try_recv() {
                wnd.handle_response(response);
            }

            screen.render(self.w)?;
        }
    }

    fn handle_action(&mut self, action: PbsAction, tx: Sender<PbsResponse>) {
        let store = self.store.clone();
        match action {
            PbsAction::Search(pattern) => {
                thread::spawn(move || {
                    let pattern = format!("%{pattern}%");
                    match store.search_items(&pattern) {
                        Ok(items) => {
                            tx.send(PbsResponse::Items(items));
                        }
                        Err(_err) => todo!(),
                    };
                });
            }
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
