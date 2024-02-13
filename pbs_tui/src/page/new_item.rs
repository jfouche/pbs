use crate::widget::{BufferAccessor, Title, Widget};

pub struct PageMakeItem;

impl Widget for PageMakeItem {
    fn display(&self, buf: &mut impl BufferAccessor) {
        buf.add(Title("Create a MAKE item".to_string()));
    }
}
