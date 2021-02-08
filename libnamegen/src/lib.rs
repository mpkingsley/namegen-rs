#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
mod xmlparse;
mod rule;

mod mchain {
#![warn(unused_mut)]
struct Chain {
    id: u32, //node id
    xmlfile: String, //filename of xml file
    markov: markov::Chain<char>,
    trained: bool
}
 
impl Chain {
    fn new(filename:&str,listid:u32, order:usize) -> Self {
        let mut markov = markov::Chain::of_order(order);
        Chain {
            id: listid,
            xmlfile: String::from(filename),
            markov,
            trained: false
        }
    }


    fn train(&mut self, data:&[String]) -> &mut Self {
        let mut view: Vec<char> = Vec::new();
        for a in data.iter() {
            let strview:Vec<char> = a.as_str().trim().chars().collect();
            for c in strview{
                view.push(c);
            }
        self.markov.feed(view.drain(0..));
     }
     self.trained = true;
     self
    }

    fn get_id(&self) -> (u32,String)  {
        (self.id,self.xmlfile.clone())
    }
    
    fn is_trained(&self) -> bool {
        self.trained
    }

}

}
use rule::Rule;
use oorandom::Rand32;
use xmlparse::Xmldoc;






