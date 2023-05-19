use webcore::value::Reference;
use webcore::try_from::TryInto;
use webapi::blob::{IBlob, Blob};

/// The File interface provides information about files and allows JavaScript
/// in a web page to access their content.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/File)
pub struct File( Reference );

impl IBlob for File {}

reference_boilerplate! {
    File,
    instanceof File
    convertible to Blob
}

impl File {
    /// Returns the name of the file referenced by the `File` object.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/File/name)
    pub fn name( &self ) -> String {
        js!( return @{self}.name; ).try_into().unwrap()
    }
}
