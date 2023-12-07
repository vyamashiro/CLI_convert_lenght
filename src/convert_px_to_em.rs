
/// Function to convert pixels to em
pub fn convert_px_to_em(x: f32) -> f32 {
    let size_parent_element: f32 = 16.0;

    println!("Converting {} px to em, considering the parent element as {} px", x, size_parent_element);
    return x / size_parent_element
}