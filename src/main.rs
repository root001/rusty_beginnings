fn main(){
    let var1 = 10;
    let varb = "X";
    let varc = "Taiwo";
    println!("Welcome to the rusty side, {} where u get {}{}", varc, var1, varb);
    mutable_variable();
}

fn mutable_variable(){
    //making a variable mutable.
    let mut var_a = 10;
    var_a = var_a / 2;
    println!("Output is : {}", var_a);
}