pub fn var_binding(){
    let x;
    let y = ["ink","has","drops"];
    x = y.len();
    println!("Binded varaible X : {} has value.", x);
}

pub fn mutable_variable(){
    //making a variable mutable.
    let mut var_a = 10;
    var_a = var_a / 2;
    println!("Mutable variable Output is : {}", var_a);
}

pub fn variable_scope(){
    //Globally available variables within this parent brace
    let varglobal = 23;
    let varspec = 221;
    {
        //Shadowing of existing variable
        let varspec = 35;
        //Locally available variable
        let newvar = 103;
        println!("Global variable is {}, scoped variables is : {} and shadowed var is : {}", varglobal, newvar, varspec);
    }
    println!("Global variable is {} and scoped variables are : {} last var is out of scope.", varglobal, varspec);
}