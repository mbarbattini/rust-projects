use std::fs::File;
use csv::Reader;
use std::error::Error;
use serde::Deserialize;
use std::io::BufReader;
use std::io::{stdin};
use std::ops::Index;

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

impl Restaurant {
    fn print_menu(&self) -> () {
        let mut i = 0;
        println!("-------- {} --------", self.name);
        for item in &self.menu {
            println!("#{} {}....... ${}", i+1, item, self.prices[i]);
            i += 1;
        }
        println!("\n");
    }

}

struct User {
    location: Vec<f32>,
    account_balance: f32
}


fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn print_restaurants_in_area(all_restaurants: &Vec<Restaurant>, user: &User) -> () {
    let user_location = &user.location;
    println!("Here are restaurants close by:");
    let mut i = 1;
    for rest in all_restaurants {    
        let distance: f32 = f32::sqrt((user_location[0] - rest.location[0]).powf(2.0) + (user_location[1] - rest.location[1]).powf(2.0));
        println!("[{}] {} ... {} miles away", i, rest.name, distance);
        i += 1;
    }
}

fn load_restaurants(filepath: &str) -> Result<Restaurant, Box<dyn Error>> {

    let restaurant_file = File::open(filepath)?;
    let reader = BufReader::new(restaurant_file); 

    let restaurants = serde_json::from_reader(reader)?;

    Ok(restaurants)
}


// impl <T, Idx> Index<Idx> for Vec<T> 
// where 
//     Idx: SliceIndex<[T], Output = T>,
// {
//     type Output = T;
//     #[inline(always)]
//     fn index(&self, index: Idx) -> &Self::Output {
//         self.slice.index(index)
//     }
// }


fn main() {

    let mut matthew = User {location: vec![1.33, -3.23], account_balance: 100.0};

    let mut all_restaurants: Vec<Restaurant>= vec![];

    let restaurant_files: Vec<&str> = vec!["assets/golden_thai.json", "assets/mcdonalds.json", "assets/brothers_bbq.json"];

    for single_restaurant in restaurant_files {
        all_restaurants.push(load_restaurants(single_restaurant).unwrap());
    }

    print_restaurants_in_area(&all_restaurants, &matthew);

    let mut choice = String::new();

    println!("Where would you like to order from? Enter `1`, `2`, ...");

    stdin().read_line(&mut choice).unwrap();

    let mut restaurant_index;
    match choice.parse::<i8>() {
        Ok(n) => restaurant_index = n,
        Err(e) => println!("Please enter a valid number!"),
    }


    // TODO. How to index a Vec<Restaurant> ???
    // let chosen_restaurant =  &all_restaurants[restaurant_index];


    // for rest in &all_restaurants {
    //     rest.print_menu();
    // }

    // print_type_of(&all_restaurants[0]);



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