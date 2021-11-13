use anx7625_2::*;

fn main() {
    let mut bus = Bus::new();
    let mut delay = Delay::new();
    let mut anx = Anx::new();
    let res = anx.init(&mut bus, &mut delay);
    println!("Res: {:?}", res);
}
