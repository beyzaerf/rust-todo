use std::collections::HashMap;
use std::io::Read;
use std::str::FromStr;

struct Todo {
    map: HashMap<String, bool>,
}

impl Todo {
    fn insert(&mut self, key:String) { // inserts into map
        self.map.insert(key, true);
    }

    fn save(self) -> Result<(), std::io::Error> { 
        let mut content = String::new();
        for (k, v) in self.map { // iterate over map
            let record = format!("{}`\t{}\n", k, v);
            content.push_str(&record) // push the formatted string into a content variable
        }
        std::fs::write("db.txt", content) // save it to db which is a txt file in this case
    }
    fn new() -> Result<Todo, std::io::Error> { //returns either Todo struct or error
        let mut f = std::fs::OpenOptions::new() // options to open the db.txt file
            .write(true)
            .create(true) // create the file if it isnt already present
            .read(true)
            .open("db.txt")?;
        let mut content = String::new();
        f.read_to_string(&mut content)?; // reads all bytes in file and appends them to content String
        let map: HashMap<String, bool> = content // convert from string to hashmap
            .lines() // creates an iterator over each line of a string
            .map(|line| line.splitn(2, '\t').collect::<Vec<&str>>()) // split lines on the tab character 
            // and transform split string to a vector of borrowed string slices
            .map(|v| (v[0], v[1])) // transform into a tuple 
            .map(|(k, v)| (String::from(k), bool::from_str(v).unwrap())) // conver the elements of tuple to string and bool
            .collect(); // collect them into our hashmap
        Ok(Todo { map }) // if there is no errors, we return our struct
    }

    fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) { // will give mutable reference to the value of key, none if not present
            Some(v) => Some(*v = false), // the * will de-reference the value and set it to false
            None => None,
        }
    }

}

fn main() {
    let action = std::env::args().nth(1).expect("Please specify an action.");
    let item = std::env::args().nth(2).expect("Please specify an item.");

    println!("{:?}, {:?}", action, item);

    let mut todo = Todo::new().expect("Initialisation of db failed");

    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("todo saved"),
            Err(why) => println!("An error occurred: {}", why),
        }
    } else if action == "complete" {
        match todo.complete(&item) {
            None => println!("'{}' is not present in the list", item),
            Some(_) => match todo.save() { // if there is a change, we update the todo
                Ok(_) => println!("todo saved"),
                Err(why) => println!("An error occurred: {}", why),
            },
        }
    }
}
