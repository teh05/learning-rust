// fn main(){
//     println!("Hello, world!");
// }


#[test]

fn main_test(){
    println!("Hello, world!");
    println!("Hello, world!");
}


#[test]
fn nama_variables(){
    let name = "mamadsss";
    println!("Hello, {}!", name);
}


#[test]
fn test_variabel(){
    let name = "Achmad Rasyid Ramadhan";
    println!("Hello, {}!", name);
}


#[test]
fn test_mutable(){
    let mut name = "Achmad Rasyid Ramadhan";
    println!("Hello, {}!", name);

    let name = "budi sadikin";
    println!("Hello, {}!", name);
}

#[test]
fn testttt_mamad(){
    let mut name = "acmad rasyid Ramadhan";
    println!("Hello, {}!", name);

    let name = "budi kopling";
    println!("Hello, {}!", name);
}




fn calculate_box_volume3(width: i32, height: i32, length: i32) -> i32 {
    width * height * length
}

fn greet_custom_message(name: &str, message: &str) {
    println!("hi {name}, {message}");
}


fn main() {
    let res3 = calculate_box_volume3(width, height, length);
    let message3 = format!("the box volume is {}", res3);

    greet_custom_message("Damian", message3.as_str());
}