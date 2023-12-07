mod convert_px_to_em;

#[cfg(test)]
mod tests {
    use crate::convert_px_to_em::convert_px_to_em;

    #[test]
    fn test_convert_px_to_em() {
        assert_eq!(convert_px_to_em(20.0), 1.25);
    }

}
