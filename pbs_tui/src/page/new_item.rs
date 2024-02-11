use crate::widget::{Buffer, Title, Widget};

pub struct PageMakeItem;

impl Widget for PageMakeItem {
    type Action = ();
    fn display(&self, buf: &mut Buffer) {
        buf.add(Title("Create a MAKE item".to_string()));
    }
}