//! Load:
//! This module has the code for loading input from file to a string buffer

// -----------------------------------------------------------------------------
// Load from source file to buffer string
// -----------------------------------------------------------------------------
pub(crate) fn data_to_buffer(file_path: String) -> crate::Result<String> {
    let buffer = std::fs::read_to_string(file_path.as_str())?;
    Ok(buffer)
}
