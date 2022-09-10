use std::cmp::Ordering;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Section {
    /// Index of where the section starts.
    offset: u16,
    /// Width begins with 0 and is the last index used
    width: u16,
}

// Accessors
impl Section {
    /// Creates a new Section.
    ///
    /// **len has to be `> 0`**
    pub fn new(offset: u16, len: u16) -> Self {
        Self {
            offset,
            width: len.checked_sub(1).unwrap(),
        }
    }
    pub fn offset(&self) -> u16 {
        self.offset
    }

    pub fn len(&self) -> u16 {
        // + 1 because `len` means the amount of chars that fit in this section,
        // as that's the way `Section` is implemented.
        self.width + 1
    }
}

// Complicated merging and separation logic
impl Section {
    pub fn contains(&self, other: &Self) -> bool {
        match self.offset.cmp(&other.offset) {
            Ordering::Less => self.offset + self.width >= other.offset + other.width,
            Ordering::Equal => self.width > other.width,
            Ordering::Greater => false,
        }
    }

    /// Return true if succeeded
    pub fn add_section(&mut self, other: &Self) -> bool {
        fn is_connected(s: &Section, o: &Section) -> bool {
            s.offset + s.width + 1 >= o.offset
        }

        match self.offset.cmp(&other.offset) {
            Ordering::Less => {
                // Test if other is contiguous or at least partially contained in self
                if is_connected(self, other) {
                    self.width = self.width.max(other.offset + other.width - self.offset);
                    true
                } else {
                    false
                }
            }
            Ordering::Equal => {
                self.width = self.width.max(other.width);
                true
            }
            Ordering::Greater => {
                if is_connected(other, self) {
                    self.width = other.width.max(self.offset + self.width - other.offset);
                    self.offset = other.offset;
                    true
                } else {
                    false
                }
            }
        }
    }

    pub fn remove_section(&mut self, other: &Self) -> RemoveResult {
        match self.offset.cmp(&other.offset) {
            Ordering::Less => {
                if self.offset + self.width <= other.offset + other.width {
                    if self.offset + self.width < other.offset {
                        RemoveResult::RemovedPart // There is no overlap between self and other, so
                                                  // self doesn't need to be changed.
                    } else {
                        // Last element should be 1 smaller than other.offset:
                        // self.offset + self.width = other.offset - 1
                        // => self.width = other.offset - self.offset - 1
                        self.width = other.offset - self.offset - 1;
                        RemoveResult::RemovedPart
                    }
                } else {
                    let offset = other.offset + other.width + 1;
                    let new_section = Section {
                        offset,
                        width: self.offset + self.width - offset,
                    };
                    self.width = other.offset - 1;
                    RemoveResult::SplitOff(new_section)
                }
            }
            Ordering::Equal => {
                if self.width <= other.width {
                    RemoveResult::Consumed
                } else {
                    // New offset begins just after the last element of other.
                    let new_offset = other.offset + other.width + 1;

                    self.offset = new_offset;
                    self.width = self.offset + self.width - new_offset; // Last element stays the
                                                                        // same. Only the distance
                                                                        // to self.offset changes.
                    RemoveResult::RemovedPart
                }
            }
            Ordering::Greater => {
                if self.offset + self.width <= other.offset + other.width {
                    RemoveResult::Consumed
                } else {
                    self.width = self.offset + self.width - other.offset - other.width - 1;
                    self.offset = other.offset + other.width + 1;
                    RemoveResult::RemovedPart
                }
            }
        }
    }
}

pub enum RemoveResult {
    /// The value was logically consumed. Though still has to be removed and dropped.
    Consumed,
    /// A Part of the value was removed. The remaining part is still valid.
    RemovedPart,
    /// A Part of the value was split of off the value and should be inserted.
    SplitOff(Section),
}
