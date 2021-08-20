#![crate_name="libnamegen"]
#![warn(clippy::needless_return)]

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
use std::time::{SystemTime, UNIX_EPOCH};

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
            i-= 1;
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

  fn contruct_doc<'a>(filename:&'a str) -> Result<Vec<Ruleset>,xmlparse::Error> {
      let mut generators:Vec<Ruleset> = Vec::new();
      let xml = Xmldoc::new(filename);
      if xml.is_err(){
          return Err(xml.unwrap_err());
      }
      let nodes = xml.unwrap().get_generators();

    return Ok(generators);
  }

  



