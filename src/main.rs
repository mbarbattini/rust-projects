// if you want to put a custom struct into a HashMap, you need to tell rust how to Hash it with the hash trait
use std::collections::HashMap;
use std::fs;
use csv::Reader;
use std::error::Error;
use serde::Deserialize;


struct FoodItem {
    name: String,
    price: String,
}

struct Restaurant {
    five_star_rating: f32,
    location: Vec<f32>,
    menu: Menu,
    name: String,
}



fn read_from_file(path: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;

    let headers = reader.headers()?;
    println!("{:?}", headers);

    for result in reader.deserialize() {
        let record: FoodItem = result?;
    
        println!("{:?}", record);
    }

    // for result in reader.records() {
    //     let record = result?;

    //     println!("{:?}", record);
    // }

    Ok(())
}

fn main() {
    let GOLDEN_THAI_MENU_ITEMS: Vec<String> = vec![
        String::from("Pad Thai"), String::from("Drunken Noodles"), String::from("Lo Mein"), String::from("Dumplings")
    ];

    let GOLDEN_THAI_MENU_PRICES: Vec<f32> = vec![
        12.99, 8.77, 14.93, 4.56
    ];

    read_from_file("src/restaurant_names.csv");



    let golden_thai_menu = Menu::new(GOLDEN_THAI_MENU_ITEMS, GOLDEN_THAI_MENU_PRICES);

    let golden_thai = Restaurant::new(vec![22.56, 11.39], golden_thai_menu, String::from("Golden Thai"));

    // let restaurant_names  = match fs::read_to_string("src/restaurant_names.txt") {
    //     Ok(restaurant_names) => restaurant_names,
    //     Err(e) => String::from("None"),
    // };
    // let restaurant_names = fs::read_to_string("src/restaurant_names.txt");


    // if let Ok(word) = restaurant_names {
    //     println!("{}", word);
    // } else {
    //     println!("")
    // }




    let mut matthewOrder = Order::default();
    let newItem = FoodItem {name: String::from("Chicken Curry"), price: 12.99};
    let newItem2 = FoodItem {name: String::from("Pad Thai"), price: 6.99};
    matthewOrder.add_item_to_cart(newItem);
    matthewOrder.add_item_to_cart(newItem2);

    matthewOrder.print_order();
}

struct Order {
    items_in_cart: Vec<FoodItem>,
    total_price: f32,
    total_items: i16,
    tax: f32,
}


impl Default for Order {
    fn default() -> Self {
        Self {
            items_in_cart: vec![],
            total_price: 0.0,
            total_items: 0,
            tax: 0.0
        }
    }
}

impl Order {
    // mutable method
    pub fn add_item_to_cart(&mut self, new_item: FoodItem) {
        self.total_price += new_item.price;
        self.items_in_cart.push(new_item);
        self.total_items += 1;
    }

    pub fn print_order(self) {
        let mut i = 1;
        for item in self.items_in_cart {
            let name = item.name;
            let price = item.price;
            println!("Item #{i} - {name}: ${price}");
            i += 1;
        }
    }
}



struct Menu {
    items: Vec<String>,
    prices: Vec<f32>,
}




// default constructor
impl Default for Restaurant {
    fn default() -> Self {
        Self {
            five_star_rating: 0.0,
            location: vec![0.0, 0.0],
            menu: Menu::default(),
            name: String::from("") 
        }
    }
}

// trait Eq implementation for FoodItem
// required to put the FoodItem in a HashMap
// docs said to include nothing
impl Eq for FoodItem {}

// implementing Eq also implies PartialEq
// if both the name, price are the same
impl PartialEq for FoodItem {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && 
        self.price == other.price 
    }
}



impl Restaurant {
    // parameterized constructor
    pub fn new(location: Vec<f32>, menu: Menu, name: String) -> Self {
        Self {
            five_star_rating: 5.0,
            location: location,
            menu,
            name,
            

        }
    }
}

// default constuctor
impl Default for Menu {
    fn default() -> Self {
        Self {
            items: vec![],
            prices: vec![],
        }
    }
}

impl Menu {
    pub fn new(items: Vec<String>, prices: Vec<f32>) -> Self {
        // let mut items_map_price = HashMap::new();
        // for item in items {
        //     items_map_price.insert(item, prices[0]);
        // }
        Self {
            items: items,
            prices: prices,
        }
    }
}

// function not applied to a struct
// fn create_menus() {
//     Menu::new(GOLDEN_THAI_MENU_ITEMS, GOLDEN_THAI_MENU_PRICES);
// }
