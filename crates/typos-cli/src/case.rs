
pub(crate) fn to_same_case(original: &str, correction: &str) -> String {
    if original.chars().all(|c| c.is_uppercase()) {
        correction.to_uppercase()
    } else if original
        .chars()
        .next()
        .map(|c| c.is_uppercase())
        .unwrap_or(false)
    {
        let mut c = correction.chars();
        c.next()
            .map(|f| f.to_uppercase().to_string() + c.as_str())
            .unwrap_or_else(|| correction.to_string())
    } else {
        correction.to_string()
    }
}
