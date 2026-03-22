//! A program that implements a coffee machine.

use core::fmt;
use std::{
    fmt::{Debug, Display},
    io,
    num::NonZeroU32,
    str::FromStr,
};
use strum_macros::{Display, EnumString};

/// The different types of coffee.
#[derive(Display, Debug, EnumString)]
#[strum(serialize_all = "lowercase")]
enum CoffeeType {
    Espresso,
    Cappuccino,
    Americano,
}

/// The different sizes of coffee.
#[derive(Display, Debug, EnumString)]
#[strum(serialize_all = "lowercase")]
enum CoffeeSize {
    Small,
    Normal,
    Large,
}

/// Represents a coffee order.
#[derive(Debug)]
struct CoffeeOrder {
    coffee_type: CoffeeType,
    coffee_size: CoffeeSize,
    sugar: Option<NonZeroU32>,
}

impl Display for CoffeeOrder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} with ", self.coffee_size, self.coffee_type)?;

        match self.sugar {
            Some(s) => write!(f, "{} sugar", s),
            None => write!(f, "no sugar"),
        }
    }
}

fn main() {
    println!("Welcome to the coffee machine!");

    println!("Enter the coffee type you want: espresso / cappuccino / americano");
    let coffee_type: CoffeeType = read_input();

    println!("Enter the coffee size you want: small / normal / large");
    let coffee_size: CoffeeSize = read_input();

    println!("Enter the amount of sugar you like");
    let sugar = read_sugar_amount();

    let order = CoffeeOrder {
        coffee_type,
        coffee_size,
        sugar,
    };
    println!("Your order is: {}", order);
}

/// Reads the sugar amount the user entered and parse it to `Option<NonZeroU32>`.
///
/// # Returns
/// The user's input as `Option<NonZeroU32>`.
fn read_sugar_amount() -> Option<NonZeroU32> {
    let sugar_amount: u32 = read_input();
    NonZeroU32::new(sugar_amount)
}

/// Receives the user's input and returns it as String.
///
/// # Returns
/// The user's input as String.
///
/// # Panics
/// If `read_line()` fails panic with `Failed to read line` message.
fn receive_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input
}

/// Receives input from the user and parse it to the wanted type.
///
/// # Returns
/// The user's input in the wanted type.
fn read_input<T: FromStr>() -> T {
    loop {
        match receive_input().trim().parse::<T>() {
            Ok(value) => return value,
            Err(_) => println!("Invalid input, please try again"),
        }
    }
}
