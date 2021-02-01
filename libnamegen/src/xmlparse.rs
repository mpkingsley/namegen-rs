use roxmltree::Document;


pub struct Xmldoc<'a> {
    file: &'a Document<'a>,
    filename: &'a str
}
pub enum Error {
    DocumentError(roxmltree::Error),
    FileReadError(std::io::Error),
}

impl From<roxmltree::Error> for Error {

    fn from(e: roxmltree::Error) -> Self {
        Error::DocumentError(e)
    }
}
impl From<std::io::Error> for Error {

    fn from(e: std::io::Error) -> Self {
        Error::FileReadError(e)
    }
}

impl<'a> Xmldoc<'a> {

 fn new(filename: &'a str ) -> Result<Self, Error> {
    let data = match read_in(filename){
        Ok(String) => String,
        Err(err) => return Err(Error::FileReadError(err)),
    };
    let xml = data.as_str().clone();
    let file = match Document::parse(xml){
        Ok(Document) => Document,
        Err(err) => return Err(Error::DocumentError(err)),
    };
    
    let doc = Xmldoc{file:&'a file,filename};

    
    Ok(doc)
 }

}

fn read_in(filename:&str) -> Result<String, std::io::Error> {
    

    return std::fs::read_to_string(filename)

}



