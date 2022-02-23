pub fn sample_print(name:String){
    println!("Hello {}, Welcome to OOP!!!", name);
}

pub fn integer_ref(){
    //declaring an i8 literal(signed) having max value as 127
    let a:i8 = -120;
    let b:u8 = 113;
    println!("Integer values are :{} {} ", a, b);
}

pub fn float(){
    //default specification for floating point numbers is f64
    let decimal_no = 23.98333333;
    let dec:f32 = 3.1427;
    println!("floating point no 32 : {} and 64 is : {}",dec, decimal_no);
}

pub fn boolean_ref(){
    let russian_fg = true;
    let fx_fg:bool = false;
    if fx_fg{
        print!("flag not activated..");
    }else{
        print!("Else exception flag not activated..");
    }
    println!("Boolean no default : {} and specified boolean : {}",russian_fg, fx_fg);
}

pub fn compound_dt(){
    //tuples
    let sample_tuple = (10, 15, 20, 12);
    println!("Tuple has values {}, {}, {}, {}", sample_tuple.0, sample_tuple.1, sample_tuple.2, 
    sample_tuple.3);

    let def_tuple:(String, bool, i32, bool) = ("kemi".to_string(), true, 20, false);
    let same_tuple:(i32,i32,i32,i32) = (11, 28, 33, 20);
    println!("Tuple: User : {}, is_Female : {}, Age : {}, Adult : {}", def_tuple.0, def_tuple.1, 
    def_tuple.2, def_tuple.3);
    println!("And Same Tuple Type as {}, {}, {}, {}", same_tuple.0, same_tuple.1, 
    same_tuple.2, same_tuple.3);

    //Arrays
    let sample_arr = [4,8,2];
    println!("Arrays: {} {} {}",sample_arr[0],sample_arr[1],sample_arr[2]);
    let alt_arr:[i32;3] = [31,8,12];
    println!("Alternatively define i32 Arrays of sze 3 as : {} {} {}",alt_arr[0],alt_arr[1],alt_arr[2]);

    let narray = (4, 2, 9);
    let (x, y, z) = narray;
    println!("Arrays: {} {} {}",x, y, z);
}
