
/// Function to convert pixels to %
pub fn convert_px_to_percentage(value_px: f32) -> f32 {
    let size_parent_element: f32 = 16.0;

    println!("Converting {} px to percentage, considering the parent element as {} px", value_px, size_parent_element);
    return (value_px / size_parent_element) * 100.0
}