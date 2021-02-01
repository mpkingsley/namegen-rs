#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
mod rule;
mod xmlparse;

use rule::Rule;
use oorandom::Rand32;

struct Chain {
    id: u32, //node id
    xmlfile: &'static str, //filename of xml file
    chain: markov::Chain<char>,
    trained: bool
}




