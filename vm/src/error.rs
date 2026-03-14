use std::process;

pub fn fatal(message: impl AsRef<str>) ->!  {
    eprintln!("{}", message.as_ref());
    process::exit(1);
}
