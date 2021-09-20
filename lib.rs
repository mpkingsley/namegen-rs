#![crate_name="libnamegen"]

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

mod xmlparse;
mod mchain;


use oorandom::Rand32;
use xmlparse::{*};
use mchain::{*};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct Document<'a> {
  generators: Vec<Ruleset>,
  lists: Vec<u32>,
  marcov: Vec<Chain>, 
  file: Xmldoc<'a>, 
}

impl<'a> Document<'a> {
  pub fn contruct_xml(filename:String) -> Result<Self,xmlparse::Error> {
    let xml = Xmldoc::new(filename.as_str());
    if xml.is_err(){
        return Err(xml.unwrap_err());
    }
    let doc = xml.unwrap();
    let generators = doc.get_generators();
    let lists: Vec<u32> = doc.get_lists();
    let mut marcov: Vec<Chain> = Vec::new();
    for id in &lists {
      let data = doc.get_data(id.clone());
      let order:usize;
      if data.len() < 15 {
        order = 1;
      }
      else if data.len() < 30  {
          order = 2
      }
      else if data.len() < 150  {
          order = 4;
      }
      else {
        order = 3;
      }
      let mut chain = Chain::new(filename.as_str(),id.clone(), order);
      chain.train(&data);
      marcov.push(chain);
    }
return Ok( Document{ generators, lists,marcov, file:xml.unwrap()})
}


    
}

pub trait Generator {
    fn marcov_generate<'a> (&self,files:&[Document<'a>]) -> Option<String>;
    fn generate<'a>(&self,files:&[Document<'a>]) -> Option<String>; 
}

impl Generator for Rule {
    fn marcov_generate<'a>(&self,files:&[Document<'a>]) -> Option<String> {
      
      None
    }

    fn generate<'a>(&self,files:&[Document<'a>]) -> Option<String> {
        //TODO 
        None
    }
}

impl Generator for Ruleset {
    fn marcov_generate<'a>(&self,files:&[Document<'a>]) -> Option<String> {
  
        None
    }

    fn generate<'a>(&self,files:&[Document<'a>]) -> Option<String> {
        
        None
    }
}

    ///selects a rule by a weighted rng. 
pub fn select_rule(ruleset: xmlparse::Ruleset) -> Rule {
  //  let mut weights:Vec<usize> = Vec::new();  //may not need this 
    let mut index:Vec<Rule> = Vec::new();
    let seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64;
    let mut rng = Rand32::new(seed);
    for r in ruleset.get_rules() {
       // weights.push(r.get_weight() as usize);
        let mut i = r.get_weight();
        while i > 0 {
            index.push(r.clone());
            i= i - 1;
        }      
    }
   return index[rng.rand_range(0.. index.len() as u32 ) as usize].clone();  
  }

  ///Seletcts an unweighted item randomly from a list. 
  pub fn select_item(list: &[u32]) -> u32 {
    let seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64;
    let mut rng = Rand32::new(seed);
    return list[rng.rand_range(0.. list.len() as u32) as usize];
  }

