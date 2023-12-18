
/// Function to convert pixels to em
pub fn convert_px_to_em(value_px: f32) -> f32 {
    let size_parent_element: f32 = 16.0;

    println!("Converting {} px to em, considering the parent element as {} px", value_px, size_parent_element);
    return value_px / size_parent_element
}