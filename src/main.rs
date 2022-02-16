mod data_types;
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
    data_types::sample_print();
    data_types::integer_ref();
    //data types
    data_types::float();
    data_types::boolean_ref();
    data_types::compound_dt();
}