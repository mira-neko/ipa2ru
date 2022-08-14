pub fn either<T>(which: bool, if_false: T, if_true: T) -> T {
    if which {
        if_true
    } else {
        if_false
    }
}
