use std::cmp::{PartialOrd, Ordering};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Position {
    pub line:   usize,
    pub offset: usize,
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Position) -> Option<Ordering> {
        Some(
            if self.line < other.line {
                Ordering::Less
            } else if self.line > other.line {
                Ordering::Greater
            } else {
                if self.offset < other.offset {
                    Ordering::Less
                } else if self.offset > other.offset {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Position;

    #[test]
    fn compare_works_when_lines_differ() {
        // Important to make the earlier position have a greater
        // offset, since that's an easy mistake to make.
        let earlier_position = Position{ line: 2, offset: 20 };
        let later_position = Position{ line: 3, offset: 10};

        assert!(earlier_position < later_position);
    }

    #[test]
    fn compare_works_when_lines_are_equal() {
        let earlier_position = Position{ line: 3, offset: 10 };
        let later_position = Position{ line: 3, offset: 20};

        assert!(earlier_position < later_position);
    }

    #[test]
    fn compare_works_when_lines_and_offsets_are_equal() {
        let earlier_position = Position{ line: 3, offset: 10 };
        let later_position = Position{ line: 3, offset: 10};

        assert!(earlier_position <= later_position);
        assert!(earlier_position >= later_position);

        // This is technically not necessary since we
        // derive the PartialEq trait, which provides
        // the implementation for this.
        assert!(earlier_position == later_position);
    }
}
