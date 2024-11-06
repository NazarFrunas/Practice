fn main() {
    const WIDTH: usize = 7;
    const HEIGHT: usize = 5;
    for i in 0..HEIGHT {
        for _ in 0..i {
            print!(" ");
        }
        print!("/");

        for _ in 0..(WIDTH - 2 * i - 2) {
            print!(" ");
        }

        if i < HEIGHT - 1 {
            print!("/");
        }
        println!();
    }

    for _ in 0..WIDTH {
        print!("-");
    }
    println!();

    for i in (0..HEIGHT).rev() {
        for _ in 0..i {
            print!(" ");
        }
        print!("\\");

        for _ in 0..(WIDTH - 2 * i - 2) {
            print!(" ");
        }

        if i < HEIGHT - 1 {
            print!("\\");
        }
        println!();
    }
}
