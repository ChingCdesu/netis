pub fn copy_string_to_cstring<T, S>(str: String) -> [T; S] {
    let mut cstring: [T; S] = [0; S];
    let mut i = 0;
    for c in str.chars() {
        cstring[i] = c as T;
        i += 1;
    }
    cstring
}
