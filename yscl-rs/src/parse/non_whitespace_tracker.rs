pub fn wrap_in_non_whitespace_tracker<I: Iterator<Item = (usize, char)>>(
    iter: I,
) -> NonWhiteSpaceTracker<I> {
    NonWhiteSpaceTracker {
        iter,
        non_whitespace_on_current_line: 0,
    }
}

#[derive(Debug, Clone)]
pub struct NonWhiteSpaceTracker<I> {
    iter: I,
    non_whitespace_on_current_line: usize,
}

impl<I> NonWhiteSpaceTracker<I> {
    pub fn non_whitespace_on_current_line(&self) -> usize {
        self.non_whitespace_on_current_line
    }
}

impl<I> Iterator for NonWhiteSpaceTracker<I>
where
    I: Iterator<Item = (usize, char)>,
{
    type Item = (usize, char);

    fn next(&mut self) -> Option<Self::Item> {
        let Some((i, c)) = self.iter.next() else {
            return None;
        };
        if c == '\n' {
            self.non_whitespace_on_current_line = 0;
        } else if !c.is_whitespace() {
            self.non_whitespace_on_current_line += 1;
        }

        Some((i, c))
    }
}
