use std::collections::BTreeMap;
use std::string::String;

struct Node {
    frequency: usize,
    next: &Node,
    childrens: BTreeMap<String, &Node>,
}

impl Node {
    pub fn new(item: String){
        Self{
            frequency: 1,
            next: None,
            childrens: BTreeMap<String, &node>::new(),
        }

    }

    pub fn show(){
        for child in Self.childrens{
            if child.show() {
                child.show()
            }else{

            }
        }
    }
}

struct Tree<T> {
    childrens: BTreeMap<String, &Node> ,
    hash_table: BtreMap<String, frequency, &node>,
}

impl Tree{
    pub fn new(hash_table: BtreMap<String, frequency, &node>){
        Self {
            childrens: BTreeMap<String, &Node>,
            hash_table: hash_table,
        }
    }

    pub fn show(){
    }
}
