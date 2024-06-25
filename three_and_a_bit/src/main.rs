// //three and a bit
// fn main(){
//     const THREE_AND_A_BIT: f32 = 3.4028236;
//     println!("{}", THREE_AND_A_BIT);
// }

//Non standard input
// use std::io::stdin;

// fn main(){
//     println!("what is  3+2. Type your answer and press enter");
//     let mut input = String::new();
//     stdin().read_line(&mut input).expect("unable to read standard input");

//     if input.trim() == "5"{
//         println!("correct!");
//     }else{
//         println!("Incorrect!");
//     }
//     println!("{:#}", input);
// }

//Type conversions
// fn main(){
//     let x:u64 = 4_294_967_296;
//     let y = x as u64;

//     if x == y as u64 {
//         println!("x equals y");
//     } else {
//         println!("x does not equal to y");
//     }
// }

//Byte sized chunks
// fn main(){
// let mut counter: i8 = 0; //2 raised to the power of 7
// loop{
//     println!("{}", counter);
//     counter +=1;
// }

// }

//How long is a string
// fn main(){
// const HELLO_WORLD: &'static str= "hello world";
//   println!("{} is {} characters long", HELLO_WORLD, HELLO_WORLD.len());
// }

// //please reboot the universe
// fn main(){
//     if 0.1f32 + 0.2f32 == 0.3f32 {
//         println!("Arithmetic still works");
//     } else {
//         println!("Rebooth the universe");
//     }
// }

//there and back again
// use std::f32::consts::PI;
// pub struct Degrees(pub f32);
// pub struct Radians(pub f32);

// impl Degrees {
//     pub fn new(angle: f32) -> Self{
//         Self(angle)
//     }
// }

// impl From<Degrees> for Radians {
//     fn from(item:Degrees) -> Self {
//         Self(item.0*PI/180.0)
//     }
// }

// fn main(){
//     let one_eigthy_degrees = Degrees::new(180.0);
//     let one_eighty_radians:Radians = one_eigthy_degrees.into();
//     println!("180 degrees in radians = {}", one_eighty_radians.0);
// }

//Walks like a Duck, Quacks like a Duck

// fn double_it(n:u64, _:i32) -> u64 {
//     n*2
// }
// fn main() {
//     let one = 1;
//     let n = double_it(one as _, 2);

//     println!("{}", n);
// }

//out of order
// fn main(){
//     let mut floats = vec![3.1, 1.2, 4.5, 0.3];
//     floats.sort_by(|a,b|a.partial_cmp(b).unwrap());
//     println!("{:?}", floats);
// }

// X matrks the spot
// fn main(){
//     if 'X'=='X'{
//         println!("It matches");
//     }else{
//         println!("it doesn't match")
//     }
// }

// //stacking boxes
// fn main() {
//     let c = vec![0u32; 10_000_000];
//     println!("{}", c.len());
// }

//Amnesia
// fn main() {
//    loop {
//     let buffer = (0..1000).collect::<Vec<u32>>();
//     std::mem::forget(buffer);
//     print!(".");
//    }
// }

//reverse the polarity of the neutron flow
// fn display_neutron_flow(polarity: isize){
//     println!(
//         "Neutron flow is {}",
//         if polarity < 0 {"reversed"} else {"normal"}
//     );
// }
// fn main(){
//     let polarity = 1;
//     {
//         let polarity = polarity - 2;
//         display_neutron_flow(polarity);
//     }
//     display_neutron_flow(polarity);
// }

//structure sizing
// use std::mem::size_of;

// struct VeryImportantMessage{
//     message_type:u8,
//     destination:u16,
// }
// fn main(){
//     println!(
//         "VeryImportantMessage occupies {} bytes",
//         size_of::<VeryImportantMessage>()
//     );
// }

//To infinity
// use std::cell::RefCell;
// use std::rc::Rc;

// type Link = Option<Rc<RefCell<Node>>>;
// //#[derive(Debug)]
// struct Node{
//     elem: i32,
//     next: Link,
// }

// use std::fmt;
// impl fmt::Debug for Node {
//     fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "elem {}", self.elem)
//     }
// }

// fn main(){
//     let mut head = Some(Rc::new(
//        RefCell::new(Node{elem: 1, next: None})
//     ));
//     head
//     .as_mut()
//     .unwrap()
//     .borrow_mut()
//     .next=Some(Rc::new(RefCell::new(
//         Node{ elem:2, next: head.clone()}
//     )));
//     println!("{:?}", head);
// }

//Double or nothing
// fn double_it<T>(n:T) -> T
// where T: std::ops::Mul <Output = T> + From<i32> {
//     n*2.into()
// }

// fn main() {
//     println!("2*4 = {}", double_it(2));
// }

//how long is a vector
// fn main(){
//     let mut my_vec = Vec::with_capacity(1);
//     my_vec.push("Hello");
//     println!("{}", my_vec.capacity());
//     my_vec.push("World");
//     println!("{}", my_vec.capacity())
// }

//Mutable Immutables
// fn main(){
//     let life_the_universe = &mut 41;
//     *life_the_universe += 1;
//     println!("Life, the Universe Everything: {}", life_the_universe);
// }

//Sleepless in Tokio
//////
////////
////////
///

//Hello, Bonjour
// fn main(){
//     let hello = || println!("Hello World");
//     let hello = || println!("Bonjour le monde");
//     hello();
// }
// #[warn(path_statements)]
// fn main() {
//     let x = 1i32;
//    println!("{}", x) ;
// }

// use std::cell::RefCell;
// fn main(){
//     let x = RefCell::new(42);
//     let y = x.borrow_mut();

//     println!("{:?}", y);
// }

// use std::thread;
// fn main() {
//         let threadhandle = thread::spawn(|| {
//                            "Hello from a thread in your Rust program"
//                                   });
//         println!("{}", threadhandle.join().unwrap());
// }

//closures
// fn call_with_three<F>(some_closure: F) -> i32 where F: Fn(i32) -> i32 {
//     some_closure(3)
// }
// fn main(){
//     let answer = call_with_three(|x| x + 10);
//     println!("{}", answer);
// }

//using closures to return a value from one of the subthreads
// use std::thread;
// fn main(){
//     let x = 10;
//     thread::spawn(move ||
//          (
//             println!("x is {}", x);
//         ));
// }

// use std::sync::{Arc, Mutex};
// use std::thread;
// use std::time::Duration;

// fn main(){
//     let primes = Arc::new(Mutex::new(vec![1, 2, 3, 5, 7, 9, 13, 17, 19, 23]));
//     for i in 1..10
//     {
//         let primes = primes.clone();
//         thread::spawn(move||{
//             let mut data = primes.lock().unwrap();
//             data[0] += 1;
//         });
//     }
//     thread::sleep(Duration::from_millis(50))
// }

// use std::thread;
// use std::sync::mpsc;
//  fn main(){
//     let (tx, rx) = mpsc::channel();
//     for i in 0..10
//     {
//         let tx =tx.clone();
//         thread::spawn(move||{
//               let answer = (i*2)*i;
//               tx.send(answer).unwrap();
//         });
//     }
//     for _ in 0..10{
//         println!("{}", rx.recv().unwrap());
//     }

//  }

// use std::thread;
// fn main(){
//   let t1  = thread::spawn(f);
//   let t2 =  thread::spawn(f);

//     println!("Hello from the main thread");
//     t1.join().unwrap();
//     t2.join().unwrap();
// }

// fn f(){
//     println!("Hello from another thread");

//     let id = thread::current().id();
//     println!("This is my thread id: {:?}", id);
// }

// use std::thread;

// fn main(){
//     let numbers = Vec::from_iter(0..=1000);
//     let t = thread::spawn(move||{
//         let len = numbers.len();
//         let sum = numbers.into_iter().sum::<usize>();
//         sum / len
//     });
//     let average = t.join().unwrap();

//     println!("the average is {}", average)
// }

// use std::rc::Rc;
// fn main(){
//     let a = Rc::new([1,2,3]);
//     let b = a.clone();

//     assert_eq!(a.as_ptr(), b.as_ptr());

//     println!("{:?}", b);
// }

/*hello boujour */

// enum Language {
//     English,
//     French
// }

// const fn hello(language:Language) -> &'static str {
//     match language {
//         Language::English => "Hello World",
//         Language::French => "Bonjour le monde",
//     }
// }

// fn main(){

//           println!("{}", hello(Language::English));

    // #[cfg(feature = "english")]
    // let hello = || println!("Hello world");

    // #[cfg(feature = "french")]
    // let hello = ||println!("Bonjour le monde");

    // hello();
//}



/* Tying a Gordian Knot */
// #[derive(Debug)]
// struct Parser<'a> {
//     body: String,
//     subtext: &'a str,
// }

// fn main() {
//     let mut document = Parser {
//         body: "Hello".to_string(),
//         subtext: "",
//     };
//     document.subtext = &document.body;

//     // let b = document;
//     println!("{:?}", document.subtext);

  
// }

//using clippy
#[warn(clippy::uninlined_format_args)]
fn main(){
    let mylist = ["one", "two", "three"];

    for item in &mylist {
        println!("{item}");
    }
}
