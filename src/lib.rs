mod convert_px_to_em;
mod convert_px_to_rem;
mod convert_px_to_percentage;

#[cfg(test)]
mod tests {
    // use super::*;
    use crate::convert_px_to_em::convert_px_to_em;
    use crate::convert_px_to_rem::convert_px_to_rem;
    use crate::convert_px_to_percentage::convert_px_to_percentage;

    #[test]
    fn test_convert_px_to_em() {
        assert_eq!(convert_px_to_em(10.0), 0.625);
    }

    #[test]
    fn test_convert_px_to_rem() {
        assert_eq!(convert_px_to_rem(10.0), 0.625);
    }

    #[test]
    fn test_convert_px_to_percentage() {
        assert_eq!(convert_px_to_percentage(10.0), 62.5);
    }

    // #[test]
    // fn test_get_typed_value() {
    //     let mut input = "16.0\n".as_bytes();
    //     io::stdin().read_line(&mut input).unwrap();
    //     assert_eq!(get_typed_value().unwrap(), 16.0);
    // }

}
