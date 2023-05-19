#[derive(Debug)]
struct HexCode{
    something: String,
}

fn something(something:String)->HexCode{
    HexCode{
        something
    }
}

fn main() {
    println!("{}",something("#120f0f".to_string()).something);
}
