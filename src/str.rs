use std::fmt;

pub fn obscure<T>(txt: T, f: &mut fmt::Formatter) -> fmt::Result
where
    T: AsRef<str>,
{
    f.write_str(&"*".repeat(txt.as_ref().len()))
}
