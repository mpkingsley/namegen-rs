#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}


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

//returns training data from a list (not a ruleset)
fn get_data(xml: &roxmltree::Document, listid: u32) -> Vec<String> {
    let mut data:Vec<String> = Vec::new();
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

pub fn generate(xml:&roxmltree::Document, ruleset: &[Rule]) -> String {
let name = String::from(iter_generate(xml,ruleset,3,1)[0].as_str());
 
name
}

pub fn iter_generate(xml:&roxmltree::Document, ruleset: &[Rule], order:usize, iter:usize) -> Vec<String>{
    use roxmltree::NodeId;

    let mut names:Vec<String> = Vec::new();
    let mut chains:Vec<Chain> = Vec::new();
    let mut count = 0;  
    loop {
        let mut name = String::new();
        let rule = rule_select(ruleset);
        println!("Generating name {: }: {: }",names.len(),rule.style);
        for idref in &rule.fmt {
            let id = find_id(xml, idref);
            match idref.as_str() {
              "space" => name.push(' '),
              "hyphen"=> name.push('-'),
              &_ => {
                 let mut trained:bool = false;
                 let mut index:usize = 0;
                for (i,c) in (&chains).iter().enumerate() {
                    if c.id == id {
                         trained = true;
                         index = i;
                         println!("Found {: } chain at index {: }", idref, i);
                    }
                }//end for
                let node = xml.get_node(NodeId::from(id)).unwrap();
                match node.tag_name().name().to_lowercase().as_str() {
                    "ruleset" => {
                      let subset = get_fmt(xml, id);
                      println!("Found Subrule at {: } ",idref );
                      println!("Generating...");
                      name.push_str(generate(xml,&subset).as_str()); 
                      println!("End Subrule Generation");
                    },
                    "list" => {
                        let data = get_data(xml,id);
                        if !trained && (data.len() >= 20) {
                            chains.push(construct_chain(xml,find_id(xml, idref),order));
                            println!("Training Markov on {: }", idref);
                            index = chains.len()-1;    
                        };
                        if data.len() > 1 && data.len() < 20  {
                            use std::time::{SystemTime, UNIX_EPOCH};                        
                            let t = SystemTime::now().duration_since(UNIX_EPOCH)
                                .expect("Time went backwards");
                            let seed = t.subsec_nanos() as u64;
                            let mut rng = oorandom::Rand32::new(seed);
                            print!("Dataset {: } is too small for Markov to work with.  ", idref);
                            let s:usize = (rng.rand_range(1..(data.len())as u32)-1) as usize ;
                            println!("Randomly selecting {: } of {: }: {: }",(s+1),data.len(),&data[s]);
                            name.push_str(&data[s]);                            
                        }
                        else if data.len() == 1 {
                            println!("Only one item in {: }: {:}",idref, &data[0]);
                            name.push_str(&data[0]);                            
                        }
                        else {
                            print!("{: } chain ", idref);
                            let text:Vec<char> = chains[index].chain.generate(); 
                            let s:String = text.into_iter().collect();
                            println!("generates: {: }", s);
                            name.push_str(&s.as_str());                            
                        }

                    },
                    &_ => (),
                } //end match 
              },
            };


                
            
        };
    while names.contains(&name) {
        println!("Duplicate name. Regenerating with fresh data.");
        name = generate(xml, ruleset);
    }
    names.push(name);
    if count < (iter-1) {
        count = count+1;
    }
    else {
        return names
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
    let alpha: Vec<String> = get_data(xml,id);
    let mut view: Vec<char> = Vec::new();
    for a in alpha.iter() {
        let strview:Vec<char> = a.as_str().trim().chars().collect();
        for c in strview{
            view.push(c);
        }
    chain.feed(view.drain(0..view.len()));
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
                 style.push_str("( )");
             },

             "hyphen" => {
                 list.push(String::from("hyphen"));
                 style.push_str("(-)");
             },

             "getlist" => {
                style.push('(');
                list.push(String::from(gchild.attribute("idref").unwrap()));
                style.push_str(gchild.attribute("title").unwrap_or("name"));
                style.push(')');
                
             },

             "getrule" => {
                style.push('(');
                list.push(String::from(gchild.attribute("idref").unwrap()));
                style.push_str(gchild.attribute("title").unwrap_or("subrule"));
                style.push(')');
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
