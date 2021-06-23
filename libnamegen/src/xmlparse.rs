#![warn(dead_code)]
use roxmltree::{*};

#[derive(Debug)]
pub struct Xmldoc<'a> {
    file: Document<'a>,
    filename: &'a str
}

#[derive(Debug)]
pub enum Error {
    DocumentError(roxmltree::Error),
    FileReadError(std::io::Error)
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
#[derive(Clone)]
pub struct Rule {
   idref: Vec<String>, //idref string
    weight: u32, //weight for each rule
    style: String,    //style with formatting
}

#[derive(Clone)]
pub struct Ruleset  {
    id: String,
    categories: Vec<String>,
    rules: Vec<Rule>
}

impl Rule{
 pub fn new(idref:Vec<String>, weight:u32, style: String) -> Rule {
     Rule{idref: idref.clone(), weight: weight.clone(), style: style.clone() }
 }

pub fn get_idref(&self) -> Vec<String> {
    self.idref.clone()
}

pub fn get_weight(&self) -> u32 {
    self.weight
}
}

impl Ruleset{
    pub fn new(id:String, categories: Vec<String>, rules: Vec<Rule> ) -> Ruleset {

        Ruleset{id:id.clone(), categories:categories.clone(),rules:rules.clone()}
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_categories(&self) -> Vec<String> {
        self.categories.clone()
    }
    pub fn get_rules(&self) -> Vec<Rule> {
        self.rules.clone()
    }

}

pub trait Generator {
    fn get_style(&self) ->String;
}
impl Generator for Rule {
    //Returns the Rule Style
    fn get_style(&self) -> String{
        self.style.clone()
    }
}

impl Generator for Ruleset {
    ///Returns a newline-delimited list of Rule styles.
    fn get_style(&self) ->String{ 
        let mut style:String = String::new();
        let mut i:u8 = 0;
        for rule in self.get_rules(){         
            i+=i;
            style.push_str(rule.get_style().as_str());
            style.push('\n');          
        }

    style
    }
}

impl<'a> Xmldoc<'a> {

 pub fn new(filename:&'a str ) -> Result<Self, Error> {
    let file = read_file(filename);
    if file.is_err() {
        return Err(file.unwrap_err());
    }
    let data = file.unwrap_or(String::from("")).into_boxed_str();
    let xml: &'a str = Box::leak(data);


    let file: Document =  match Document::parse(xml) {
        Ok(d) => d,
        Err(err) => return Err(Error::DocumentError(err)),
    };
    
    let doc = Xmldoc{file,filename};


    Ok(doc)
 
 }
 pub fn get_title(&self, id: u32) -> Option<String> {
     let xml = &self.file;
     let node = xml.get_node(NodeId::from(id));
     if node.is_none(){
         return None;
     }
     let title = node.unwrap().attribute("title");
     if title.is_none() {
         return None;
     }
     return Some(String::from(title.unwrap()))
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
  ///returns the list of "final" rulesets
 pub fn get_generators(&self) -> Vec<Ruleset>{
     let xml = &self.file;
     let rulesets = self.get_rulesets();
     let mut generators:Vec<Ruleset> = Vec::new();
     for r in rulesets {
         let node = xml.get_node(NodeId::from(r as usize)).unwrap();
         let mut categories:Vec<String> = Vec::new();
         let mut rules: Vec<Rule> = Vec::new();
         if node.has_attribute("usage") && (node.attribute("usage").unwrap() == "final" ){
            let id = String::from(node.attribute("title").unwrap());            
            for child in node.children(){
                if child.has_tag_name("Categories") {
                    categories.push(String::from(child.attribute("title").unwrap()));
                }
                else if child.has_tag_name("Rule") {
                    rules.push(self.get_rule(child.id().get() as u32));

                }

         }
         generators.push(Ruleset::new(id, categories, rules))
         }
     }
     
     return generators;
 }

 pub fn find_id(&self, idref: &str) -> u32 {
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

 pub fn get_lists(&self) -> Vec<u32> {
    let xml = &self.file;
    let mut ids:Vec<u32> = vec!();
    for node in xml.descendants(){
        if node.tag_name().name().to_lowercase() == "list" {
            ids.push(node.id().get());
        }
    }
    ids
  }
  ///Constructs a Rule from the given node
 pub fn get_rule(&self, node_id: u32) -> Rule {
    let xml = &self.file;
    let node = xml.get_node(NodeId::from(node_id)).unwrap();
    let mut idref:Vec<String> = Vec::new();
    let mut style:String = String::new();
    let mut weight:u32 = 1;
    for value in node.children() {
        match value.tag_name().name().to_lowercase().as_str(){
          "space" => {
             idref.push(String::from("space"));
             style.push_str("( )");
         },
        
         "hyphen" => {
            idref.push(String::from("hyphen"));
            style.push_str("(-)");
         },

         "getlist" => {
           style.push('(');
           idref.push(String::from(value.attribute("idref").unwrap()));
           style.push_str(value.attribute("title").unwrap_or("name"));
           style.push(')');
           
          },
        
        "getrule" => {
           style.push('(');
           idref.push(String::from(value.attribute("idref").unwrap()));
           style.push_str(value.attribute("title").unwrap_or("subrule"));
           style.push(')');
        },
        _ => (), //match all not prior
        }; //end match
                
     }; //end for
        let w = node.attribute("weight").unwrap_or("1");
        if w.chars().collect::<Vec<char>>()[0].is_numeric(){
        let trim:String = w.chars().filter(|d| d.is_digit(10)).collect();
        weight = trim.parse::<u32>().unwrap();

    }
    return Rule::new(idref.clone() ,weight, style.clone());
 }

  ///Retreives each item from the list. 
 pub fn get_data(&self, listid: u32) -> Vec<String> {
    let mut data:Vec<String> = Vec::new();
    let xml = &self.file;
    let list = xml.get_node(roxmltree::NodeId::from(listid)).unwrap();
    for value in list.children(){
        if value.tag_name().name().to_lowercase() == "value" {
            let text = value.text().unwrap_or("Parsing_Err").trim();
            data.push(String::from(text));
           // println!("Found data: {: }", text);
        }

    }
    data
  }

}

pub fn read_file<'a> (filename:&'a str) -> Result<String, Error> {
    let read = std::fs::read_to_string(filename);
    match read {
        Ok(data) => {
            return Ok(data)
        },
        Err(error) => return Err(Error::FileReadError(error)),
    };

}