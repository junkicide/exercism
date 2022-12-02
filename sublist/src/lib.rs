use num::signum;

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list == second_list {
        return Comparison::Equal
    }
    else {
        let flen = first_list.len() as i32;
        let slen = second_list.len() as i32;
        let diff = signum(flen - slen);
        match diff {
            0 => return Comparison::Unequal,
            1 => {
                if is_sublist(first_list, second_list) {
                    return Comparison::Sublist
                }
                else {
                    return Comparison::Unequal
                        
                }
            }
            -1 => {
                if is_sublist(second_list, first_list) {
                    return Comparison::Superlist
                }
                else {
                    return Comparison::Unequal
                }
            }
            _ => Comparison::Unequal
        }
    }
}

fn is_sublist<T: PartialEq> (first: &[T], second: &[T]) -> bool {
    let iter = second.windows(first.len());
    for i in iter {
        if i == first {
            return true;
        } 
    }
    false
}
