#[derive(Clone,Copy)]
pub struct Rule<'a> {
    idref: &'a[String], //idref string
    weight: u32, //weight for each rule
    style: &'a str,    //style with formatting
}

trait Rules {
    
 fn new(idref:&[String]) -> Self;
 fn get_style(&self) -> &str;   
 fn get_weight(&self) -> u32 ;
 fn get_idref(&self) -> &[String];
}
