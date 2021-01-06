// use std::io;
// use std::fs;
#![warn(clippy::needless_return)]

extern crate markov;
extern crate roxmltree;
extern crate random_pick;
extern crate oorandom;

#[derive(Clone)]
pub struct Rule {
    fmt: Vec<String>, //idref string
    weight: u32, //weight for each idref
    style: String,    //style with formatting
}

struct Chain {
    id: u32, //node id
    chain: markov::Chain<char>,
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
        std::io::stdin().read_line(&mut ctrl).expect("Error Reading from stdin"); 
        print!("\x1B[2J\x1B[1;1H"); //clear the screen
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
                for rule in namefmt.clone() {
                    println!("{: }, weight: {: }", rule.style, rule.weight);
                    println!("Includes the following lists:");
                    for list in rule.fmt {
                        println!("     {: }", list);
                    }
                }
                ctrl = String::new();
                println!("How many names to generate?");
                std::io::stdin()
                    .read_line(&mut ctrl)
                    .expect("Error Reading from stdin"); 
                ctrl = ctrl.chars().filter(|d| d.is_digit(10)).collect();
                let iter = ctrl.parse::<usize>().unwrap_or(1);
                print!("\x1B[2J\x1B[1;1H"); //clear the screen
                println!("Starting Name Generation:");
                let names = iter_generate(&xmldoc, &namefmt, iter);
                for (i,name) in names.iter().enumerate(){
                    
                    println!("name {: }: {: }",i+1,name);
                };
                

                
            }
            else {
                    println! ("Invalid selection: {: }",index);
                    continue;
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
    ids
  }

fn get_rulesets(xml: &roxmltree::Document) -> Vec<u32> {
  let mut ids:Vec<u32> = vec!();
  for node in xml.descendants(){
      if node.tag_name().name().to_lowercase() == "ruleset" {
          ids.push(node.id().get());
      }
  }
  ids
}
//formats the Ruleset id in a prettier format
fn fmt_menu(xml: &roxmltree::Document, id: u32) -> String {
    let mut pretty  = String::new();
    let node = xml.get_node(roxmltree::NodeId::from(id)).unwrap();
    let mut descr:Vec<&str> = Vec::new();
    pretty.push_str(node.attribute("title").unwrap());
    pretty.push(':'); 
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
    pretty
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
 0 //return 0 if not found
}

//returns training data String and number of items from a list (not a ruleset)
fn get_tdata(xml: &roxmltree::Document, listid: u32) ->  (String,u32) {
    let mut tdata = (String::new(),0);
    let list = xml.get_node(roxmltree::NodeId::from(listid)).unwrap();
    for value in list.children(){
        if value.tag_name().name().to_lowercase() == "value" {
            let text = value.text().unwrap_or("").trim();
            tdata.0.push_str(text);
            tdata.1 = tdata.1 + 1 ;
        }
        tdata.0.push(' ');//whitespace separator for training

    }
tdata
}

fn get_data(xml: &roxmltree::Document, listid: u32) -> Vec<String> {
    let mut data:Vec<String> = Vec::new();
    let list = xml.get_node(roxmltree::NodeId::from(listid)).unwrap();
    for value in list.children(){
        if value.tag_name().name().to_lowercase() == "value" {
            let text = value.text().unwrap_or("Parsing Err").trim();
            data.push(String::from(text));
            println!("Found data: {: }", text);
        }
        else {
            println!("Tag found with name: {: }",(value.tag_name().name().to_lowercase()));
        };
    }
data
}

pub fn generate(xml:&roxmltree::Document, ruleset: &[Rule]) -> String {
let name = String::from(iter_generate(xml,ruleset,1)[0].as_str());
 
name
}

pub fn iter_generate(xml:&roxmltree::Document, ruleset: &[Rule], iter:usize) -> Vec<String>{
    use roxmltree::NodeId;

    let mut names:Vec<String> = Vec::new();
    let mut chains:Vec<Chain> = Vec::new();
    let mut count = 1;
    let order = 3; //Order of three seems to be suffient for name generation.  
    loop {
        names.push(String::new());
        let rule = rule_select(ruleset);
        for idref in &rule.fmt {
            let id = find_id(xml, idref);
            match idref.as_str() {
              "space" => names[count-1].push(' '),
              "hyphen"=> names[count-1].push('-'),
              &_ => {
                 let mut trained:bool = false;
                 let mut index:usize;
                for (i,c) in (&chains).iter().enumerate() {
                    if c.id == id {
                         trained = true;
                         index = i;
                         break;
                    }
                }//end for
                let node = xml.get_node(NodeId::from(id)).unwrap();
                match node.tag_name().name().to_lowercase().as_str() {
                    "ruleset" => {
                      let subset = get_fmt(xml, id);
                      println!("Found Subrule at {: } ",idref );
                      println!("Generating...");
                      names[count-1].push_str(generate(xml,&subset).as_str()); 
                    },
                    "list" => {
                        let data = get_data(xml,id);
                        if !trained && (data.len() > 25) {
                            chains.push(construct_chain(xml,find_id(xml, idref),order));
                            index = chains.len()-1;    
                        };
                        if data.len() > 1 && data.len() <= 25  {
                            use std::time::{SystemTime, UNIX_EPOCH};                        
                            let t = SystemTime::now().duration_since(UNIX_EPOCH)
                                .expect("Time went backwards");
                            let seed = t.subsec_nanos() as u64;
                            let mut rng = oorandom::Rand32::new(seed);
                            print!("Dataset {: } is too small for Markov to work with.  ", idref);
                            let s:usize = (rng.rand_range(1..(data.len())as u32)-1) as usize ;
                            println!("Randomly selecting {: } of {: }: {: }",s,data.len(),&data[s]);
                            names[count-1].push_str(&data[s]);                            
                        }

                    },
                    &_ => (),
                } //end match 
              },
            };
                
            
        };
    if count == iter {
    return names;
    }
    else{
        names.push(String::new());
        count = count+1;
    }
    }






}

fn rule_select(ruleset: &[Rule]) -> &Rule {
  use random_pick::gen_usize_with_weights;
  let mut weights:Vec<usize> = Vec::new();
  for r in ruleset {
      weights.push(r.weight as usize);
  }
   let index = gen_usize_with_weights(ruleset.len(), &weights).unwrap();
   let rule = &ruleset[index];

  &rule
}

fn construct_chain(xml: &roxmltree::Document, id:u32, order:usize ) -> Chain {
 let mut chain = markov::Chain::of_order(order);
 println!("Collecting training data");
 let alpha: Vec<char> = get_tdata(xml,id).0.chars().collect();
 let mut view: Vec<char> = Vec::new();
 for a in alpha {
     view.push(a);
     if a == ' ' {
         chain.feed(view.drain(0..view.len()));
     }

 }
 Chain {id, chain}
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
   return Rule { fmt:list, weight:ruleweight, style};
}

