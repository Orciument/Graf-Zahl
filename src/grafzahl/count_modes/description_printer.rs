use crate::grafzahl::count_modes::countable::Countable;

pub(crate) fn print_description<CountMode: Countable>() {
    CountMode::display_description();
}