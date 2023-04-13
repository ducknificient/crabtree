

pub fn second_function(){
    //println!("This is from second function");
    ownership();
}


fn ownership(){
    let mut s = String::from("Hello");

    s.push_str(", world!"); // push_str() appends a literal to a string

    println!("{}", s);
}
