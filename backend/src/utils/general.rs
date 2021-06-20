// /// prints function's name
// ///https://stackoverflow.com/questions/38088067/equivalent-of-func-or-function-in-rust
// macro_rules! function {
//     () => {{
//         fn f() {}
//         fn type_name_of<T>(_: T) -> &'static str {
//             std::any::type_name::<T>()
//         }
//         let name = type_name_of(f);
//
//         // Find and cut the rest of the path
//         match &name[..name.len() - 3].rfind(':') {
//             Some(pos) => &name[pos + 1..name.len() - 3],
//             None => &name[..name.len() - 3],
//         }
//     }};
// }

pub fn type_name_of<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}
