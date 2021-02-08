use roxmltree::Document;

pub struct Xmldoc<'a> {
    file: Document<'a>,
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

 fn new(data: &'a str, filename:&'a str ) -> Result<Self, Error> {

    let file: Document<'a>;
    file = match Document::parse(data){
                Ok(document) => document,
                Err(err) => return Err(Error::DocumentError(err)),
            };
    let doc = Xmldoc{file:file,filename};

    Ok(doc)
 
 }

 fn get_rulesets(&self) -> Vec<u32> {
    let mut ids:Vec<u32> = vec!();
    let xml = &self.file;
    for node in xml.descendants(){
        if node.tag_name().name().to_lowercase() == "ruleset" {
            ids.push(node.id().get());
        }
    }
    ids
  }

  fn find_id(&self, idref: &str) -> u32 {
    use roxmltree::NodeId;
    let xml = &self.file;
    let lists = &self.get_lists();
    let rulesets = &self.get_rulesets();
 
    for id in lists {
        let list =  xml.get_node(NodeId::from(*id)).unwrap();
        if list.attribute("id").unwrap_or("none") == idref {
            return *id
        };
    }
    for id in rulesets {
        let rule =  xml.get_node(NodeId::from(*id)).unwrap();
         if rule.attribute("id").unwrap_or("none") == idref {
             return *id
         };
     }
  0 //return 0 if not found
 }

 fn get_lists(&self) -> Vec<u32> {
    let xml = &self.file;
    let mut ids:Vec<u32> = vec!();
    for node in xml.descendants(){
        if node.tag_name().name().to_lowercase() == "list" {
            ids.push(node.id().get());
        }
    }
    ids
  }

  fn get_data(&self, listid: u32) -> Vec<String> {
    let mut data:Vec<String> = Vec::new();
    let xml = &self.file;
    let list = xml.get_node(roxmltree::NodeId::from(listid)).unwrap();
    for value in list.children(){
        if value.tag_name().name().to_lowercase() == "value" {
            let text = value.text().unwrap_or("Parsing Err").trim();
            data.push(String::from(text));
           // println!("Found data: {: }", text);
        }
        /* else {
            println!("Tag found with name: {: }",(value.tag_name().name().to_lowercase()));
        }; */
    }
data
}

}

pub fn read_file(filename:&str) -> Result<String, Error> {
    match std::fs::read_to_string(filename){
       Ok(data) => return Ok(data),

       Err(error) => return Err(Error::FileReadError(error)),
   };
}