fn main() {
    let x = 5;
    let y = {
        let z = 6;
        x + z
    };
    println!("The value of y is {y}");

    let w = another_function();
    println!("The value of w is {w}");
    measurement(5, 'm');
}

fn another_function() -> i32{
    println!("Hey, this is the another function!");
    return 5;
}

fn measurement(value : i32, unit : char){
    println!("The measured value is {value}{unit}");
}
