#[derive(Clone,Copy)]
pub struct Rule<'a> {
    idref: &'a[String], //idref string
    weight: u32, //weight for each rule
    style: &'a str,    //style with formatting
}

trait Rules<'a> {
 fn get_style(&self) -> &str;   
 fn get_weight(&self) -> u32 ;
 fn get_idref(&self) -> &[String];
}

impl<'a> Rules<'a> for Rule<'a> {
    fn get_style(&self) -> &str{
            self.style.clone()
    }
    fn get_idref(&self) -> &[String] {
        self.idref.clone()
    }
    fn get_weight(&self) -> u32 {
        self.weight
    }
}


impl<'a> Rule<'a>{
    fn new(idref: &'a [String], weight:u32, style: &'a str) -> Rule<'a> {
        Rule{idref: idref.clone(), weight: weight.clone(), style: style.clone() }
    }
}

