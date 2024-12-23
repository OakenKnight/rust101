use std::io;
use std::process;
mod player;
use crate::archive::arch::arch_file as arch;
use rand::Rng;
mod archive;

const URL: &str= "test";
fn user_input(){
    let mut input = String::new();
    println!("Say something: ");
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("You said: {}", input);
        },
        Err(e) => {
            println!("Error message: {}", e); 
        }
    }
}
fn print_stuff(){
    println!("My name is {} and I'm {} years old", "Alex", 26);
    println!("a + b = {}", 3 + 6);
    println!("{0}, has a {2} and {0} has a {1}", "Alex", "dog", "cat");
    println!("Hello everyone, my name is {name} and surname is {surname}", name="Alex", surname="Ignjatijevic" );
    println!("bin: {:b}, oct: {:o} and hex: {:x}", 50, 50, 50);
    println!("Array {:?}", [1,2,3,4]);
}
#[allow(unused_variables)]
fn lang_basics(){
    let name: &str = "Alex"; //scalar type rather than String which is object
    let mut age: i32 = 32;

    let amount:i64 = 1928634918264981264;
    println!("age before change: {}", age);
    age = 23;
    println!("age after change: {}", age);

    let color= "blue";
    println!("color before change: {}", color);
    let color= 0x89CFF0;
    println!("color after change: {:x} in hex", color);

    let (a, b, c) = (43, 85, "red");
    println!("{}, {}, {}", a, b, c);

}
#[allow(unused_variables)]
#[allow(unused_assignments)]
fn scalar_types(){
    let pi: f32 = 3.14;
    let milion = 1_000_000;
    println!("{}", milion);

    let is_day = true;
    let is_night = false;
    println!("{}", is_day);
    let char1 = 'A';
    let char1 = '\u{1F601}';
    println!("{}",char1);
}
#[allow(unused_variables)]
#[allow(unused_assignments)]
fn strings(){
    let cat: &'static str = "Fluffy"; //static will live in the scope of the function or whoever is calling the function
    println!("{}", cat);
    //string object is different and can be modified
    let dog = String::new(); //static will live in the scope of the function or whoever is calling the function
    let dog = String::from("Alfi");
    println!("{}", dog);

    let mut owner = format!("Hi everyone, I am {} owner of {}", "Alex", dog);
    println!("{}\nlength of str: {}", owner, owner.len());
    owner.push(' ');
    owner.push_str("the dog.");
    println!("{}\nlength of str: {}", owner, owner.len());
    let new_owner = owner.replace(".", "!");
    println!("{}", new_owner);

}
fn say_hello(name: &mut &str) -> String{
    let greeting = format!("Hello {}", name);
    greeting
}
fn functions(){
    let mut name = "John";
    let greeting = say_hello(&mut name);
    println!("{}", greeting);
}
fn test_player(){
    player::play_audio("random audio");
    player::play_movie("random movie");
    clean::perform_clean();
    clean::files::clean_files();
}
mod clean {
    pub fn perform_clean() {
        println!("cleaning hdd");
    }
    pub mod files {
        pub fn clean_files() {
            println!("Removing unused files");
        }
    }
}
fn crates(){
    arch("test file");

    let mut rng = rand::thread_rng();
    let a: i32 = rng.gen();
    println!("{}", a);
}

#[derive(Debug)]
#[allow(dead_code)]
struct Point<T, V> {
    x: T,
    y: V
}
#[derive(Debug)]
enum Colors<T>{
    Red(T),
    Green(T),
    Blue(T),
}
fn generics(){
    let p1: Point<i32, f32> = Point { x: 6, y: 8.0 };
    let p2: Point<f32, &str> = Point { x: 6.218734, y: "8.543" };
    println!("{:?}", p1);
    println!("{:?}", p2);
    let c1 = Colors::Red("#f00");
    let c2 = Colors::Blue(255);
    let c3 = Colors::Green(255);

    println!("{:?}", c1);
    println!("{:?}", c2);
    println!("{:?}", c3);

}
fn pattern_matching(){
    for i in 0..15 {
        println!("{}) I have {} oranges", i, get_oranges(i));
    }

}
fn get_oranges(amount: i32) -> &'static str{
    match amount {
        0 => "no",
        1 | 2 => "one or two",
        3..7 => "a few",
        _ if(amount % 2 == 0 ) => "even number of",
        _ => "lots of"
    }
}
fn for_loops(){
    let pets = ["alfi", "max", "fluffy", "betty", "helly"];
    for pet in pets.iter(){
        if pet == &"alfi" {
            println!("My dog is {}", pet);
            continue;
        }
        if pet == &"helly" {
            break;
        }
        println!("{}", pet);
    }

    for (pos, i) in (1..11).enumerate(){
        let sq = i * i;
        let nb = pos + 1;
        println!("{0} * {0} = {1}", nb, sq);
    }
}
static mut R: i32 = 9;
#[allow(static_mut_refs)]
fn functions2() {
    let mut name = "John";
    say_hi(&mut name);
    println!("Hi2 {}", name);
    {
        let a = 32;
        println!("{}", a);
    }
    //println!("{}",a); fails
    //println!("{}", R);  this is unsafe
    unsafe{
        R = 4;
        println!("R= {}", R); 
    }
    unsafe{
        println!("R= {}", R); 
    }
}
fn say_hi(name: &mut &str) {
    *name = "Alex";
    println!("Hi {}", name);
}
fn closures() {
    let a = |a:i32| a + 1;
    println!("{}",a(5));
    let b = |b: i32| -> i32 {
        let c = a(5) + 1;
        c + b
    };
    println!("{}",b(5430));
    let gen = |x| println!("{}", x);
    gen(54);
    //gen(true); throws error because firstly it was called with integer, now every function afterwards will be recognized as integer
}
fn apply(f: fn(i32) -> i32, a: i32){
    println!("Result: {}",f(a));

}
fn is_even(x: i32) -> bool {
    x % 2 == 0
}
fn hof(){
    let square = |a: i32| a * a;
    apply(square,6);

    //Calcualte the sum of all the squares less than 500
    //only 4 even numbers
    let limit = 500;
    let mut sum = 0;
    for i in 0..limit {
        let isq = i * i;
        if isq > limit {
            break;
        }
        else {
            if is_even(isq){
                sum += isq
            }
        }
    }
    println!("The sum is: {}", sum);

    // with hof
    let sum2 = 
            (0..).map(|x| x * x)
                 .take_while(|&x| x <= limit)
                 .filter(|x| is_even(*x))
                 .fold(0, |sum, x| sum + x);
    
    println!("The sum2 is: {}", sum2);

}
fn main() {
    println!("Welcome to mastering rust course from Udemy!");
    let mut input = String::new();
    while input != "0" {
        println!("Here is what we got so far for you:");
        println!("1) User input");
        println!("2) Printing stuff");
        println!("3) Language basics");
        println!("4) Scalar types");
        println!("5) Strings");
        println!("6) Constant");
        println!("7) Functions");
        println!("8) Modules");
        println!("9) Crates");
        println!("10) Generics");
        println!("11) Pattern matching");
        println!("12) For loops");
        println!("13) Functions 2");
        println!("14) Closures");
        println!("15) HOF");
        println!("------------------\n0) Exit");
        println!("Choose which part of lesson you want to see: ");
        io::stdin().read_line(&mut input).expect("Something went wrong");
        match input.trim().parse() {
            Ok(opt) => {
                match opt {
                    1 => { user_input(); },
                    2 => { print_stuff(); },
                    3 => { lang_basics(); },
                    4 => { scalar_types(); },
                    5 => { strings(); },
                    6 => { println!("{}", URL); },
                    7 => { functions(); },
                    8 => { test_player(); },
                    9 => { crates(); },
                    10 => { generics(); },
                    11 => { pattern_matching(); },
                    12 => { for_loops(); },
                    13 => { functions2(); },
                    14 => { closures(); },
                    15 => { hof(); },

                    0 => { process::exit(0); },
                    50..100 => { println!("Range with match"); }
                    _ => { println!("Wait for it.."); }
                }
            },
            Err(e) => { println!("error parsing user input: {:?}", e)}
        }
        input = String::new();
    }
    
   
}
