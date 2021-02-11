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







