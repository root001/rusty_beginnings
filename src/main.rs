mod data_structure;

fn main(){
    let var1 = 10;
    let varb = "X";
    let varc = "Taiwo";
    println!("Welcome to the rusty side, {} where u get {}{}", varc, var1, varb);
    mutable_variable();
    // making a call to a method outside the method scope
    data_structure::sample_print();
}

fn mutable_variable(){
    //making a variable mutable.
    let mut var_a = 10;
    var_a = var_a / 2;
    println!("Output is : {}", var_a);
}