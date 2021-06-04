pub fn pick_first_option_available<T>(a: Option<T>, b: Option<T>, c: Option<T>) -> Option<T> {
    match a {
        Some(a) => Some(a),
        None => match b {
            Some(b) => Some(b),
            None => match c {
                Some(c) => Some(c),
                None => None,
            },
        },
    }
}
