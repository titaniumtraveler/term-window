use crate::window::Id;
use std::{
    fmt::{self, Write},
    ops::RangeBounds,
};

pub use pane::Pane;

mod pane;

pub trait Render {
    fn id(&self) -> &Id<'_>;
    fn id_mut(&mut self) -> &mut Id<'_>;

    /// Render/Rerender the full pane.
    fn render(&mut self, pane: Pane<impl Write>) -> fmt::Result;

    /// Render only a collumn of the pane.
    fn render_line(&mut self, mut pane: Pane<impl Write>, line: usize) -> fmt::Result {
        let mut buf = String::new();
        self.render(Pane::new(pane.shape(), &mut buf))?;
        if let Some(line) = buf.lines().nth(line) {
            pane.write_fmt(format_args!("{line}\n"))?;
        }
        Ok(())
    }

    /// Render a range of lines.
    fn render_range(
        &mut self,
        mut pane: Pane<impl Write>,
        range: impl RangeBounds<usize>,
    ) -> fmt::Result {
        let mut buf = String::new();
        self.render(Pane::new(pane.shape(), &mut buf))?;

        for line in get_range(buf.lines(), range) {
            pane.write_fmt(format_args!("{line}\n"))?;
        }
        Ok(())
    }
}

/// Gets a range of items from an iterator.
fn get_range<I: Iterator<Item = Item>, Item, R: RangeBounds<usize>>(
    iterator: I,
    range: R,
) -> impl Iterator<Item = Item> {
    let start_bound = match range.start_bound() {
        std::ops::Bound::Included(&num) => num,
        std::ops::Bound::Excluded(&num) => num + 1,
        std::ops::Bound::Unbounded => 0,
    };

    let mut end_bound = match range.end_bound() {
        std::ops::Bound::Included(&num) => Some(num + 1),
        std::ops::Bound::Excluded(&num) => Some(num),
        std::ops::Bound::Unbounded => None,
    };

    iterator
        .take_while(move |_| {
            if let Some(num) = &mut end_bound {
                if *num == 0 {
                    false
                } else {
                    *num -= 1;
                    true
                }
            } else {
                true
            }
        })
        .skip(start_bound)
}
