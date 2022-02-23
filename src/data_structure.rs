// TO DO .. recurssion in rust??
pub fn number_sum(mut x:i32)->i32{
    let mut result = 0;
    while x > 0 {
        result =  result + x;
        x = x - 1;
    //    println!("Value for X is {} and current sum is {}. ", x, result);
    }
    return result;
} 

pub fn sample_loop(){
    let mut i = 0;
    loop {
        println!("Value of i is : {}", i);
        if i > 10 {
            break;
        }
        i += 1;
    }
}

pub fn sample_forloop(){
    // array declaration and for each loop initiated
    let list_arr = [1, 3, 6, 5, 28];
    // TO DO ..getting the index of an array from the value or array object??
    for elmnt in list_arr.iter(){
        println!("The xth element in the array has value : {}", elmnt)
    }
}