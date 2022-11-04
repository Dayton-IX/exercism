#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    let mut result = Comparison::Unequal;
    let mut i = 0;
    if _first_list == _second_list {
        result = Comparison::Equal;
        return result;
    }
    for item in _first_list.iter() {
        if item == &_second_list[i] {
            result = Comparison::Sublist;
        } else if result == Comparison::Sublist && item != &_second_list[i] {
            result = Comparison::Sublist;
            return result;
        } else {
            result = Comparison::Unequal;
        }
        i += 1;
    }
    if result == Comparison::Sublist { 
        if _first_list.len() > _second_list.len() {
            result = Comparison::Superlist
        }
    }
    return result;
}

fn check_sublist<T: ParialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    unimplemented("Not ready yet");
}

fn check_superlist<T: ParialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    unimplemented("Not ready yet");
}
