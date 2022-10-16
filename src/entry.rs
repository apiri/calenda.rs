use std::ops::Sub;
use time::PrimitiveDateTime;

#[derive(Debug)]
pub struct Entry {
    start: PrimitiveDateTime,
    end: PrimitiveDateTime,
}

impl Entry {
    pub fn new(start: PrimitiveDateTime, end: PrimitiveDateTime) -> Self {
        Entry { start, end }
    }

    pub fn start(&self) -> &PrimitiveDateTime {
        &self.start
    }

    pub fn end(&self) -> &PrimitiveDateTime {
        &self.end
    }

    pub fn duration_mins(&self) -> i64 {
        self.end.sub(self.start).whole_minutes()
    }
}
