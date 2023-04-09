use std::io;
use std::task::ready;

fn main() {
    fizzbuzz();
}

// Write a program that reads in a list of integers and prints out the average of the integers.
fn average_numbers() {
    let mut input = String::new();
    println!("Enter a list of integers separated by spaces:");
    io::stdin().read_line(&mut input).unwrap();

    let numbers: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Not a number"))
        .collect();

    let sum: i32 = numbers.iter().sum();
    println!("Average:{}", sum as f32 / numbers.len() as f32);
}

// Write a program that reads in a list of integers and prints out a new list that contains only the even integers from the original list.
fn filter_even_list() {
    let mut input = String::new();
    println!("Enter a list of integers separated by spaces:");
    io::stdin().read_line(&mut input).unwrap();

    let even_numbers: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Not a number"))
        .filter(|n| n % 2 == 0)
        .collect();

    println!("Even numbers are: {:?}", even_numbers);
}

// Write a program that reads in a list of strings and prints out the shortest and longest strings in the list.
fn shortest_longest_string() {
    let mut input = String::new();
    println!("Enter a list of strings separated by spaces:");
    io::stdin().read_line(&mut input).unwrap();

    let strings: Vec<String> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Not a string"))
        .collect();


    let max = strings.iter().max().unwrap();
    let min = strings.iter().min().unwrap();
    println!("Max: {}, Min: {}", max, min);
}

// Write a program that reads in a list of integers and prints out the sum of all the even integers in the list.
fn sum_numbers() {
    let mut input = String::new();
    println!("Enter a list of integers separated by spaces:");
    io::stdin().read_line(&mut input).unwrap();

    let numbers: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Not a number"))
        .collect();

    let sum: i32 = numbers.iter().sum();
    println!("Sum:{}", sum);
}

// Write a program that reads in a list of integers and prints out the largest and smallest integers in the list.
fn largest_smallest_list() {
    let mut input = String::new();
    println!("Enter a list of integers separated by spaces:");
    io::stdin().read_line(&mut input).unwrap();

    let numbers: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Not a number"))
        .collect();

    let min = numbers.iter().min().unwrap();
    let max = numbers.iter().max().unwrap();

    println!("min: {}, max: {}", min, max);
}


// Write a program that reads in a string and prints out the string in reverse order.
fn reverse_string() {
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).unwrap();
    let mut reversed_string = String::new();

    for c in input_string.chars() {
        reversed_string.insert(0, c);
    }

    // Alternatively using native rev() function
    // let reversed_string: String = input_string.chars().rev().collect();

    println!("Reversed String: {}", reversed_string);
}

// Write a program that reads in two numbers and prints their sum, difference, product, and quotient.
fn math_operations() {
    let mut input_string = String::new();

    println!("Enter number 1:");
    io::stdin().read_line(&mut input_string).unwrap();
    let num1: i32 = input_string.trim().parse().expect("Must be a number");

    input_string.clear();

    println!("Enter number 2:");
    io::stdin().read_line(&mut input_string).unwrap();
    let num2: i32 = input_string.trim().parse().expect("Must be a number");

    println!("Sum of numbers: {}", num1 + num2);
    println!("Difference of numbers: {}", num1 - num2);
    println!("Product of numbers: {}", num1 * num2);
    println!("Quotient of numbers: {}", num1 / num2);
}

// Write a program that takes in a number from the user and prints whether it is even or odd.
fn even_or_odd() {
    let mut num_string = String::new();
    println!("Enter a number");
    while let Err(_) = io::stdin().read_line(&mut num_string) {
        println!("Failed to read input. Please try again.");
    }
    let num: i32 = num_string.trim().parse().expect("Not a number");

    if num % 2 == 0 {
        println!("Even");
    } else {
        println!("Odd");
    }
}

// Write a program that reads in a user's name and greets them. For example, if the user enters "John", the program should print "Hello, John!".
fn greet() {
    let mut name = String::new();
    println!("Enter a name:");
    while let Err(_) = io::stdin().read_line(&mut name) {
        println!("Failed to read input. Please try again.");
    }
    name = name.trim().to_string();

    while name.is_empty() || !name.chars().all(|c| c.is_alphabetic()) {
        println!("Invalid name. Try again:");
        name.clear();
        io::stdin().read_line(&mut name).unwrap();
        name = name.trim().to_string();
    }

    println!("Hello {}", name.trim());
}

// Write a program that prints the numbers from 1 to 100. For multiples of three, print "Fizz" instead of the number and for multiples of five, print "Buzz". For numbers that are multiples of both three and five, print "FizzBuzz".
fn fizzbuzz() {
    for i in 1..=100 {
        match (i % 3, i % 5) {
            (0, 0) => println!("{}: fizzbuzz", i),
            (0, _) => println!("{}: fizz ", i),
            (_, 0) => println!("{}: buzz", i),
            _ => println!("{}", i),
        }
    }
}