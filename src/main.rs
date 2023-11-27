use std::fs::File;
use csv::Reader;
use std::error::Error;
use serde::Deserialize;
use std::io::BufReader;


// need to modify the Cargo.toml file to derive Deserialize in serde
#[derive(Deserialize, Debug)]
struct Restaurant {
    name: String,
    number_items: i32,
    menu: Vec<String>,
    prices: Vec<f32>,
    five_star_rating: f32,
    location: Vec<f32>,
}



fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}



fn load_restaurants(filepath: &str) -> Result<Restaurant, Box<dyn Error>> {

    let restaurant_file = File::open(filepath)?;
    let reader = BufReader::new(restaurant_file); 

    let restaurants = serde_json::from_reader(reader)?;

    Ok(restaurants)
}





fn main() {
    


    // let mut matthew_order = Order::default();
    // let new_item = FoodItem {name: String::from("Chicken Curry"), price: 12.99};
    // let new_item_2 = FoodItem {name: String::from("Pad Thai"), price: 6.99};
    // matthew_order.add_item_to_cart(new_item);
    // matthew_order.add_item_to_cart(new_item_2);



    let mut all_restaurants: Vec<Restaurant>= vec![];

    // matthew_order.print_order();
    let restaurant_files: Vec<&str> = vec!["assets/golden_thai.json", "assets/mcdonalds.json"];

    for single_restaurant in restaurant_files {
        all_restaurants.push(load_restaurants(single_restaurant).unwrap());
    }
    

    print_type_of(&all_restaurants[0]);



}

struct FoodItem {
    name: String,
    price: f32,
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