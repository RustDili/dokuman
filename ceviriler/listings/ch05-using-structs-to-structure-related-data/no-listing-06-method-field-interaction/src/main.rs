#[derive(Debug)]
struct Dikdortgen {
    width: u32,
    height: u32,
}

// ANCHOR: here
impl Dikdortgen {
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let dik1 = Dikdortgen{
        width: 30,
        height: 50,
    };

    if rect1.width() {
        println!("Dikdörtgen sıfır olmayan bir ene sahip;  değeri {}", dik1.width);
    }
}
// ANCHOR_END: here
