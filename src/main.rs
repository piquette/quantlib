use quantlib::bonds::Bond;

fn main() {
    let b = Bond { ytm: 40 };

    println!("b: {:?}", b);
}
