// use std::io;
// use std::fs;
#![warn(clippy::needless_return)]

extern crate markov;
extern crate roxmltree;
extern crate random_pick;
extern crate rand;

#[derive(Clone)]
pub struct Rule {
    fmt: Vec<String>, //idref string
    weight: u32, //weight for each idref
    style: String,    //style with formatting
}

struct Chain {
    id: u32,
    chain: markov::Chain<char>,
}

fn main() {  
    //TODO: parse args for filename (and potential output file)

    //Load and parse file:
    let filename = "roman.xml";
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

//returns training data String and number of items from a list
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

pub fn generate(xml:&roxmltree::Document, rule: &Rule) -> String {
let mut name = String::new();
let mut idlist:Vec<u32> = Vec::new();
let mut chains:Vec<Chain> = Vec::new();
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
                  if tdata.1 < 25 {
                    use rand::distributions::{Distribution, Uniform};
                     let list = xml.get_node(NodeId::from(id)).unwrap();
                     if tdata.1 == 1{
                        let text=list.first_child().unwrap().text().unwrap_or("error in xml parsing").trim();
                        println!(" Only one entry in {: }: {: }",idref,text);
                        name.push_str(text);
                        continue;
                     }
                     let mut rng = rand::thread_rng();
                     let n = Uniform::from(1..tdata.1);
                     let j = n.sample(&mut rng);
                     print!("{: } is too small for markov to work with. Randomly selecting number {: } from {: } items:", idref, j, tdata.1);
                     for (i,value) in list.children().enumerate() {
                        if (value.tag_name().name().to_lowercase() == "value") && i== j as usize {
                            let text=value.text().unwrap().trim();
                            println!(" {: }",text);
                            name.push_str(text);
                        }

                     };
                  }
                  else {
                      if !idlist.contains(&id){
                        idlist.push(id);   
                        let order = 3;  //could possible unhardcode order here.  need convinsing reason to do so. Personal testing indicates that 3 is the best order for shorter sequences like names, etc
                        println! ("training Markov on {: }", idref);
                        chains.push(construct_chain(xml,id,order)); 
                        println! ("Training concluded"); 
                      }
                      let m = idlist.iter().position(|p| p == &id).unwrap();
                      for c in chains[m].chain.generate(){
                          if c != ' ' {
                              name.push(c);
                          }
                      }   
                  }//end else
                 },
               &_ => (),
            }
        }
       },
     }
    }
name
}

pub fn iter_generate(xml:&roxmltree::Document, ruleset: &[Rule], iter:usize) -> Vec<String>{
    //TODO Refactor so each unique chain is only generated and trained once.
    let mut names:Vec<String> = Vec::new();
    let mut i = 0;
     while i < iter {
         let rule = rule_select(ruleset);
         println!("Generating name {: }: {: }", (i + 1) , rule.style);
         names.push(generate(xml, rule));
         i = i +1;

    }
    names
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

