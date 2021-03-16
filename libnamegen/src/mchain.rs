extern crate oorandom;
extern crate markov;

//use std::collections::HashMap;

struct Chain {
        id: u32, //node id
        filename: String, //filename of xml file
        markov: markov::Chain<char>,
        trained: bool
    }
     
    impl Chain {
       pub fn new(filename:&str,listid:u32, order:usize) -> Self {
            let mut markov = markov::Chain::of_order(order);
            Chain {
                id: listid,
                filename: String::from(filename),
                markov,
                trained: false
            }
        }

        pub fn train(&mut self, data:&[String]) -> &mut Self {
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
    
        pub fn get_id(&self) -> (u32,String)  {
            (self.id,self.filename.clone())
        }
        
        pub fn is_trained(&self) -> bool {
            self.trained
        }

        pub fn generate(&self) -> Vec<char> {
            let chain = &self.markov;
            return chain.generate();
        }

        
    
    }

    
