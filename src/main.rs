// use std::io;
// use std::fs;

extern crate markovr;
extern crate roxmltree;
extern crate random_pick;
extern crate rand;

pub struct Rule {
    fmt: Vec<String>, //idref string
    weight: u32, //weight for each idref
    style: String,    //style with formatting
}


fn main() {  
    //TODO: parse args for filename (and potential output file)

    //Load and parse file:
    let filename = "gothic.xml";
    let xmldata = std::fs::read_to_string(filename).unwrap();
    let xmldoc = roxmltree::Document::parse(&xmldata).unwrap();
    let rulesets =  get_rulesets(&xmldoc);

    loop {
        let mut c: char; //loop control char.
        let mut ctrl  = String::new(); 
        println!("The File {:?} contains the following types:",filename);
        println!("Please select your option: or q to quit");
        for (i,id) in rulesets.iter().enumerate() {
            let alpha = &xmldoc.get_node(roxmltree::NodeId::from(*id)).unwrap();
            if alpha.attribute("usage").unwrap_or("empty").to_lowercase() == "final" {
                let entry = fmt_menu(&xmldoc,*id);
                let printing:Vec<&str> = entry.split('\t').collect();
                print!("{:?}: ",i+1);
                for desc in printing {
                    println!("{: }",desc);
                }
            }
        }
        std::io::stdin()
            .read_line(&mut ctrl)
            .expect("Error Reading from stdin"); 
        c =  ctrl.chars().collect::<Vec<char>>()[0];
        if c.is_numeric() {
            ctrl = ctrl.chars().filter(|d| d.is_digit(10)).collect();
            print!("You chose {: }: ", ctrl);
            let index = ctrl.parse::<usize>().expect("Invalid entry");

            if index <= (rulesets.len())  {
                let mut selection = fmt_menu(&xmldoc, rulesets[index-1]);
                selection = String::from(selection.split('\t').collect::<Vec<&str>>()[0]);
                println!("{: }",selection);
                let namefmt = get_fmt(&xmldoc, rulesets[index-1]);
                for rule in namefmt {
                    println!("{: }, weight: {: }", rule.style, rule.weight);
                    println!("Includes the following lists:");
                    for list in rule.fmt {
                        println!("     {: }", list);
                    }
                }
                println!("How many names to generate?");
                std::io::stdin()
                    .read_line(&mut ctrl)
                    .expect("Error Reading from stdin"); 
                ctrl = ctrl.chars().filter(|d| d.is_digit(10)).collect();
                let iter = ctrl.parse::<usize>().expect("Invalid entry");
                
                

                
            }
            else {
                    println! ("Invalid selection: {: }",index);
            }
        }
        else {
            c.make_ascii_lowercase();
        }
        if c == 'q' {
            break;
        }

    }
    println!("Bye!");
}

fn get_lists(xml: &roxmltree::Document) -> Vec<u32> {
    let mut ids:Vec<u32> = vec!();
    for node in xml.descendants(){
        if node.tag_name().name().to_lowercase() == "list" {
            ids.push(node.id().get());
        }
    }
    return ids
  }

fn get_rulesets(xml: &roxmltree::Document) -> Vec<u32> {
  let mut ids:Vec<u32> = vec!();
  for node in xml.descendants(){
      if node.tag_name().name().to_lowercase() == "ruleset" {
          ids.push(node.id().get());
      }
  }
  return ids
}
//formats the Ruleset id in a prettier format
fn fmt_menu(xml: &roxmltree::Document, id: u32) -> String {
    let mut pretty  = String::new();
    let node = xml.get_node(roxmltree::NodeId::from(id)).unwrap();
    let mut descr:Vec<&str> = Vec::new();
    pretty.push_str(node.attribute("title").unwrap());
    pretty.push_str(":"); 
    for child in node.children(){
      if child.tag_name().name().to_lowercase() == "category" {
        let title = child.attributes()[0].value();
        if title.to_lowercase().contains("sex:"){ 
            //add Gender to the name
          pretty.push_str(title.trim_start_matches("Sex:"));
        }
        else if title.to_lowercase().contains("all"){
            //do nothing
        }
        else {
         descr.push(title);
          }
    }
    }
    for item in descr {
        pretty.push('\t');
        pretty.push_str("     ");
        pretty.push_str(item);
    }
    return pretty
}

fn get_fmt(xml: &roxmltree::Document, id: u32) -> Vec<Rule> {
  let node = xml.get_node(roxmltree::NodeId::from(id)).unwrap();
  let mut ruleset:Vec<Rule> = Vec::new(); 
  for child in node.children(){
    if child.tag_name().name().to_lowercase() == "rule" {  
        ruleset.push(get_rule(xml,child.id().get()));    
    };//end if
  };//end for
  
  ruleset
}

fn find_id(xml: &roxmltree::Document, idref: &str) -> u32 {
   let lists = get_lists(xml);
   let rulesets = get_rulesets(xml);

   for id in lists {
       let list =  xml.get_node(roxmltree::NodeId::from(id)).unwrap();
       if list.attribute("id").unwrap_or("none") == idref {
           return id
       };
   }
   for id in rulesets {
       let rule =  xml.get_node(roxmltree::NodeId::from(id)).unwrap();
        if rule.attribute("id").unwrap_or("none") == idref {
            return id
        };
    }
   return 0
}

//returns training data String and number of items from a list
fn get_tdata(xml: &roxmltree::Document, listid: u32) ->  (String,u32) {
    let mut tdata = (String::new(),0);
    let list = xml.get_node(roxmltree::NodeId::from(listid)).unwrap();
    for value in list.children(){
        if value.tag_name().name().to_lowercase() == "value" {
            tdata.0.push_str(value.text().unwrap_or(""));
            tdata.1 += tdata.1 ;
        }
        tdata.0.push(' ');//whitespace separator for training
    }
tdata
}

pub fn generate(xml:&roxmltree::Document, rule: &Rule) -> String {
let mut name = String::new();
for idref in &rule.fmt {
    match idref.as_str() {
     "space" => name.push(' '),
     "hyphen"=> name.push('-'),
     &_ =>{            
        let id = find_id(xml, &idref);
        if id == 0 {
            eprintln!("ID not found; please check xml file");
        }
        else{
            use roxmltree::NodeId;
            let node = xml.get_node(NodeId::from(id)).unwrap();
            match node.tag_name().name().to_lowercase().as_str() {
              "ruleset" => {
                let subset = get_fmt(xml, id);
                let subrule = rule_select(&subset);
                name.push_str(generate(xml,&subrule).as_str());
                },
              "list" => {
                  let tdata = get_tdata(xml,id);
                  if tdata.1 < 20 {
                     use rand::{thread_rng, Rng};
  
                     let mut rng = thread_rng();
                     let n: u32 = rng.gen_range(0..tdata.1);
                     let list = xml.get_node(NodeId::from(id));
                     //TODO get n  in list
                  }
                  else {
                      //todo create & generate using marcov 
                  }




              },
              &_ => (),
            }
        }
       },
     }
    }

return name
}

pub fn iter_generate(xml:&roxmltree::Document, rule: &Rule, iter:usize) -> Vec<String>{
    let mut names:Vec<String> = Vec::new();
    let i = 0;
     while i < iter {
         names.push(generate(xml, rule));

    }
    return names
}

fn rule_select(ruleset: &Vec<Rule>) -> &Rule {
  use random_pick::gen_usize_with_weights;
  let mut weights:Vec<usize> = Vec::new();
  for r in ruleset {
      weights.push(r.weight as usize);
  }
   let index = gen_usize_with_weights(ruleset.len(), &weights).unwrap();
   let rule = &ruleset[index];
  
    return rule
}

fn construct_chain(tdata:String, order:usize ) -> markovr::MarkovChain<char> {
 use markovr::MarkovChain;
 let mut chain = MarkovChain::new(order, &[]);
 let alpha: Vec<char> = tdata.chars().collect();
 for i in 1..alpha.len() {
     chain.train(&[alpha[i-order]], alpha[i], 1);
 }

 chain
}

fn get_rule(xml: &roxmltree::Document, id: u32) -> Rule {
  let mut list: Vec<String> = Vec::new();
  let mut style = String::new();
  let mut ruleweight:u32 = 1;
  let child = xml.get_node(roxmltree::NodeId::from(id)).unwrap();
  for gchild in child.children() {
        match gchild.tag_name().name().to_lowercase().as_str(){
             "space" => {
                 list.push(String::from("space"));
                 style.push(' ');
             },

             "hyphen" => {
                 list.push(String::from("hyphen"));
                 style.push('-');
             },

             "getlist" => {
                list.push(String::from(gchild.attribute("idref").unwrap()));
                style.push_str(gchild.attribute("title").unwrap_or("unknown"));
                
             },

             "getrule" => {
                list.push(String::from(gchild.attribute("idref").unwrap()));
                style.push_str(gchild.attribute("title").unwrap_or("unknown"));
             },
             _ => (), //match all not prior
        }; //end match
        
    }; //end for
    let w = child.attribute("weight").unwrap_or("1");
    if w.chars().collect::<Vec<char>>()[0].is_numeric(){
        let trim:String = w.chars().filter(|d| d.is_digit(10)).collect();
        ruleweight = trim.parse::<u32>().expect("Unexpected xml data.  Please check your sourcefile");    
    };
  let namestyle = Rule { fmt:list, weight:ruleweight, style:style};
  namestyle
}

