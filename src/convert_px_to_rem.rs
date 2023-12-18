
/// Function to convert pixels to rem
pub fn convert_px_to_rem(value_px: f32) -> f32 {
    let size_root_element: f32 = 16.0;

    println!("Converting {} px to rem, considering the root element as {} px", value_px, size_root_element);
    return value_px / size_root_element
}