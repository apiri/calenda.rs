use std::ops::{Add, Sub};
use time::macros::datetime;
use time::{Duration, PrimitiveDateTime};

#[derive(Debug)]
pub struct Entry {
    start: i64,
    end: i64,
}

pub const BEGINNING_OF_TIME: PrimitiveDateTime = datetime!(1970-01-01 00:00:00);

impl Entry {
    pub fn new(start: PrimitiveDateTime, end: PrimitiveDateTime) -> Self {
        let start = Entry::as_minutes(start);
        let end = Entry::as_minutes(end);

        Entry { start, end }
    }

    pub fn start(&self) -> PrimitiveDateTime {
        Entry::to_primitive_date_time(self.start)
    }

    pub fn end(&self) -> PrimitiveDateTime {
        let seconds = Duration::new(self.end * 60, 0);
        BEGINNING_OF_TIME.add(seconds)
    }

    pub fn duration_mins(&self) -> i64 {
        self.end - self.start
    }

    pub fn as_minutes(date_time: PrimitiveDateTime) -> i64 {
        date_time.sub(BEGINNING_OF_TIME).whole_minutes()
    }

    pub fn to_primitive_date_time(minutes: i64) -> PrimitiveDateTime {
        let seconds = Duration::new(minutes * 60, 0);
        BEGINNING_OF_TIME.add(seconds)
    }
}

#[derive(Default)]
pub struct Entries {
    entries: Vec<Entry>,
}

impl Entries {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Provides an ordered collection of all entries.
    /// Eventually this will provide a virtual view of overlapping, merged entries
    pub fn entries(&self) -> &Vec<Entry> {
        &self.entries
    }

    /// Adds an entry to the system
    pub fn add_entry(&mut self, entry: Entry) {
        self.entries.push(entry);
    }
}

#[cfg(test)]
mod tests {
    use crate::entry::Entry;
    use time::macros::datetime;

    #[test]
    fn test_entry_as_minutes() {
        assert_eq!(Entry::as_minutes(datetime!(2022-10-04 12:00)), 27748080);
        assert_eq!(
            Entry::as_minutes(datetime!(2022-10-03 12:00)),
            27748080 - 24 * 60
        );
    }

    #[test]
    fn test_entry_to_primitive_date_time() {
        assert_eq!(
            Entry::to_primitive_date_time(27748080),
            datetime!(2022-10-04 12:00)
        );
    }
}
