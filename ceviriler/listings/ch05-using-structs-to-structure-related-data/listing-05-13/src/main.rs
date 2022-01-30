#[derive(Debug)]
struct dikdortgen {
    width: u32,
    height: u32,
}

impl dikdortgen {
    fn alan(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let dik1 = dikdortgen{
        width: 30,
        height: 50,
    };

    println!(
        "dikdörtgenin alanı {} kare pikseldir.",
        rect1.alan()
    );
}
