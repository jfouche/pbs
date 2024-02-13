mod main_wnd;
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
use main_wnd::MainWindow;
use pbs_core::{Item, Store};
use screen::Screen;

pub enum PbsAction {
    Search(String),
    CreateItem(String),
    ViewItem(i64),
}

pub enum PbsResponse {
    Err(String),
    Items(Vec<Item>),
    Item(Item),
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

                    Event::Resize(cols, rows) => screen.resize(cols as usize, rows as usize),

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
                    let response = match store.search_items(&pattern) {
                        Ok(items) => PbsResponse::Items(items),
                        Err(err) => PbsResponse::Err(format!("{err:?}")),
                    };
                    tx.send(response).expect("Invalid thread state");
                });
            }
            PbsAction::CreateItem(name) => {
                thread::spawn(move || {
                    let response = match store.make_item(&name) {
                        Ok(item) => PbsResponse::Item(item),
                        Err(err) => PbsResponse::Err(format!("{err:?}")),
                    };
                    tx.send(response).expect("Invalid thread state");
                });
            }
            PbsAction::ViewItem(id) => {
                thread::spawn(move || {
                    let response = match store.item(id) {
                        Ok(_item) => match store.children(id) {
                            Ok(_children) => unimplemented!(),
                            Err(err) => PbsResponse::Err(format!("{err:?}")),
                        },
                        Err(err) => PbsResponse::Err(format!("{err:?}")),
                    };
                    tx.send(response).expect("Invalid thread state");
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
            cursor::SetCursorStyle::BlinkingUnderScore
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

#[cfg(test)]
mod test {
    use std::ops::Range;

    struct Buffer {
        v: Vec<u8>,
    }

    struct BufferView<'a> {
        b: &'a mut Buffer,
        range: Range<usize>,
    }

    trait BufferAccessor {
        fn view(&mut self, range: Range<usize>) -> BufferView;
    }

    impl BufferAccessor for Buffer {
        fn view(&mut self, range: Range<usize>) -> BufferView {
            BufferView { b: self, range }
        }
    }
    impl BufferAccessor for BufferView<'_> {
        fn view(&mut self, range: Range<usize>) -> BufferView {
            BufferView { b: self.b, range }
        }
    }

    #[test]
    fn test() {
        let mut buf = Buffer { v: vec![1, 100] };
        let bv = buf.view(0..2);

        // let bv = BufferView {
        //     b: &mut buf,
        //     range: 0..2,
        // };
    }
}
