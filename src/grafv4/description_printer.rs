use crate::grafv4::countable::Countable;

pub(crate) fn print_description<CountMode: Countable>() {
    CountMode::display_description();
}