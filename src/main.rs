mod data_types;
mod variable_fcn;
mod data_structure;

fn main(){
    let var1 = 10;
    let varb = "X";
    let varc = "Taiwo";
    println!("Welcome to the rusty side, {} where u get {}{}", varc, var1, varb);
    variable_fcn::mutable_variable();
    variable_fcn::var_binding();
    variable_fcn::variable_scope();
    // making a call to a method outside the method scope
    data_types::sample_print("Loni".to_string());
    data_types::integer_ref();
    //data types
    data_types::float();
    data_types::boolean_ref();
    data_types::compound_dt();
    println!("Function Calls and returns: the total sum of input is : {}", data_structure::number_sum(5));
    data_structure::sample_loop();
    data_structure::sample_forloop();
}