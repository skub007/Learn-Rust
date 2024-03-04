// We import a library to handle input and output operations.
use std::io;

fn main() {
    // We create a variable x and set its initial value to "yoho".
    let mut x: &str = "str - yoho";

    // Inside these curly braces is a 'block' of code.
    // Any changes made to variables inside {} are temporary.
    {
        // We print the current value of x.
        println!("x cur is: {}", x);

        // We change the value of x to "2378".
        x = "2378";

        // We print the updated value of x.
        println!("x upd. is: {}", x);

        // We declare a new variable x with the value 33.33.
        let x: f64 = 33.33;

        // We print this new value of x.
        println!("x float is: {}", x);
    }

    // After exiting the block, x retains its original value.
    println!("org. val. x - {}", x);

    // We create a variable a and set it to 4.
    let mut a: i32 = 4;

    // We print the initial value of a.
    println!("a init. is: {}", a);

    // Inside these curly braces is another 'block' of code.
    {
        // We increase the value of a by 102.
        a = a + 102;

        // We declare a new variable a with the value "hello".
        //a can be set to string inside {}
        let a = "hello";

        // We print this new value of a.
        println!("a new value is: {}", a);
    }

    // After exiting the block, a returns to its final value.
    println!("a org. is: {}", a);

    // We create a constant named B and set it to "yo".
    const B: &str = "yo";  

    // We print the value of this constant B.
    println!("b const is: {}", B);

    // We declare a tuple without assigning any values yet.
    let tup: (i32, &str, char, f32);

    // We now assign values to this tuple.
    tup = (2, "hello", 'a', 3.1);

    // We print specific elements of the tuple using indexing.
    println!("tup val 3 is: {}", tup.3);
    println!("tup val 0 is: {}", tup.0);
    println!("tup val 1 is: {}", tup.1);
    println!("tup val 2 is: {}", tup.2);

    // Input section
    // We create a variable called name to store user input.
    let mut name = String::new();
    println!("Your Name :- ");
    // We read input from the user and store it in the name variable.
    io::stdin().read_line(&mut name).expect("Failed to read the line...");

    // We print the input provided by the user.
    println!("your name is {}", name);

    //maths!!
    //*ITS RECOMMENDED NOT TO EXPLICITLY TYPE THE VARIABLE LIKE -> let a : i8 = 2; instead do let a = 2; */
    let n1 = 4;
    let n2 = 4;
    let _re = 0;
    //add
    {
        //again.. temp values
        let re = n1+n2;
        println!("Add - {}",re);
    }
    //perma. values
    let re = n1+n2;
    println!("Add - {}",re);

    //substraction
    {
        //temp ->
        let re = n1-n2;
        println!("Subs. - {}",re);
    }
    //perma.
    let re = n1-n2;
    println!("Subs. - {}",re);

    //multiply
    {
        //temp ->
        let re = n1*n2;
        println!("multiply - {}",re);
    }
    //perma.
    let re = n1*n2;
    println!("multiply - {}",re);

    //Divide
    {
        //temp ->
        let re = n1/n2;
        println!("Divide - {}",re);
    }
    //perma.
    let re = n1/n2;
    println!("Divide - {}",re);
    //There are more similer to python like
    // % - Return remainder
    // ""//"" - (no "" is there its for clarification) floor division
    // look for more by searching arithmetics in python 

    //parsing {converting} string to int
    let r = "1000";
    let sdd : i64 = r.trim().parse().unwrap();
    println!("{}",sdd+100);

    //operators
    // -> 1.) > 2.) < 3.) <= 4.) >= 5.) ==
    //conditional statements -> 1.) if 2. if...else if..else 3.) else
    //if ->
    let c1 = 100;
    let c2 = 10;
    if c1 > c2 {
        println!("-> {}",c1);
        print!(" is greater than {} ",c2);
    }
    //if else
    let c3 = 10;
    let c4 = 100;
    if c3 > c4 {
        println!("{}",c3);
        println!(" is greater than {} ",c4);
    }
    else{
        println!("{}",c3);
        println!(" is smaller than {} ",c4);
    }
    //if..else if...else
    let c5 = "hu";
    if c5 == "huu" {
        println!("if running hu");
    }
    else if c5 == "hu"{
        println!("else if running huu");
    }
    else{
        println!("else running ooh ok");
    }
    //cll ur custom func using its followed by ();
    fnone();
    usefunc(32,32.32);

    //btw, expresions ->
    let dnd = 5^4 + 400;
    println!("{}",dnd);
    let vcb = return_fn(32,64);
    println!("{}",vcb);

}

//Declare functions using the fn keywork like gn my_func(){}
fn fnone(){
    println!("running from fnone fn i am fnone!!");
}

//custom functions can have arguments
fn usefunc(x:i32 , f:f32){
    println!("int {}",x);
    println!("float {}",f);

}
//returning vslues from funcs
fn return_fn(x:i32,y:i32) -> i32 /* using -> we specify which value it return */{
    //no semicolon
    //to use semicolon use return keywork like this :- return x+y;
    x+y
}