use std::process;

// pub fn fatal(message: impl AsRef<str>) ->!  {
//     eprintln!("{}", message.as_ref());
//     process::exit(1);
// }

#[macro_export]
macro_rules! fatal {
    ($($arg:tt)*)  => {
        eprintln!($($arg)*);
        process::exit(1);
    };
}