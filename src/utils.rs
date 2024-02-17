pub fn parse_validated<T>(s: &str) -> T
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    s.parse().expect("pattern should be validated by caller")
}
