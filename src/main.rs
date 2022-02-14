mod data_structure;
mod variable_fcn;

fn main(){
    let var1 = 10;
    let varb = "X";
    let varc = "Taiwo";
    println!("Welcome to the rusty side, {} where u get {}{}", varc, var1, varb);
    variable_fcn::mutable_variable();
    variable_fcn::var_binding();
    variable_fcn::variable_scope();
    // making a call to a method outside the method scope
    data_structure::sample_print();
    data_structure::integer_ref();
}