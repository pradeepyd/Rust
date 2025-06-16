fn main() {
    println!("Hello, world!");
    let x: i8 = 5;
    let y:u32 = 500;
    let z:f32 = 100.002;

    print!("x: {},y : {}, z: {}",x,y,z);

    let is_male :bool = false;
    let is_above_18 : bool = true;
    if is_male && is_above_18 {
        println!("you are a male");
    } else {
         println!("you are not male");
    }

    let greeting : String = String :: from("hello world");
    println!("{}", greeting);

    let mut x :String = String :: from ("hello my name is Pradeep");
    x.push_str("first word is {}");

}
