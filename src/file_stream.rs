use std;

pub struct FileStream {
    base: InputStream,

    filename: String,
}

impl FileStream {
    fn new(fileName: String) -> Result<FileStream, core::io::Error> {
        unimplemented!()
    }

    fn get_source_name(&self) -> String {
        unimplemented!()
    }
}
