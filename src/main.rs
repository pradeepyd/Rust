fn main() {
    println!("Hello, world!");
    let x: i8 = 5;
    let y:u32 = 500;
    let z:f32 = 100.002;

    print!("x: {},y : {}, z: {}",x,y,z);

    let is_male :bool = false;
    let is_above_18 : bool = true;
    if is_male {
        println!("you are a male");
    } else {
         println!("you are not male");
    }
}
