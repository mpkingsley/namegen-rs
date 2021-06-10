#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
mod xmlparse;
mod rule{

#[derive(Clone,Copy)]
pub struct Rule<'a> {
   idref: &'a[String], //idref string
    weight: u32, //weight for each rule
    style: &'a str,    //style with formatting
}

impl<'a> Rule<'a>{
 fn new(idref: &'a [String], weight:u32, style: &'a str) -> Rule<'a> {
     Rule{idref: idref.clone(), weight: weight.clone(), style: style.clone() }
 }

 pub fn get_style(&self) -> &str{
    self.style.clone()
}

pub fn get_idref(&self) -> &[String] {
    self.idref.clone()
}

pub fn get_weight(&self) -> u32 {
    self.weight
}
}


}
mod mchain;
use rule::Rule;
use oorandom::Rand32;
use xmlparse::Xmldoc;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn select_rule<'a>(ruleset: &'a[Rule<'a>]) -> &Rule<'a> {
  //  let mut weights:Vec<usize> = Vec::new();  //may not need this 
    let mut index:Vec<&Rule<'a>> = Vec::new();
    let seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64;
    let mut rng = Rand32::new(seed);
    for r in ruleset {
       // weights.push(r.get_weight() as usize);
        let mut i = r.get_weight();
        while i > 0 {
            index.push(&r);
            i= i - 1;
        }

      
    }

   return index[rng.rand_range(0 .. index.len() as u32 ) as usize];  
  }






