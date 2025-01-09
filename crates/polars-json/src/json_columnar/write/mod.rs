//! APIs to write to JSON
mod serialize;
mod utf8;

use std::io::Write;
pub use fallible_streaming_iterator::*;
use polars_core::frame::DataFrame;
use polars_error::PolarsResult;
pub(crate) use serialize::new_serializer;
pub use utf8::serialize_to_utf8;

/// Writes valid JSON from an iterator of (assumed JSON-encoded) bytes to `writer`
pub fn write<W>(writer: &mut W, df: &DataFrame) -> PolarsResult<()>
where
    W: Write,
{
    writer.write_all(b"{")?;
    let mut is_first_column = true;
    for series in df.iter() {

        if !is_first_column {
            writer.write_all(b",")?;
        }
        is_first_column = false;

        writer.write_all(b"\"")?;
        writer.write_all(series.name().as_ref())?;
        writer.write_all(b"\":[")?;

        let mut is_first_row = true;

        for chunk in series.chunks() {
            let mut array_serializer =  new_serializer(chunk.as_ref(), 0, usize::MAX);

            while let Some(block) = array_serializer.next() {
                if !is_first_row {
                    writer.write_all(b",")?;
                }
                is_first_row = false;
                writer.write_all(block)?;
            }
        }

        writer.write_all(b"]")?;
    }
    writer.write_all(b"}")?;
    Ok(())
}
