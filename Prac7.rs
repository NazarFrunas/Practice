fn main() {
    let num_triangles = 5; 
    for i in 0..num_triangles {
        let height = i + 1;
        let width = 2 * height - 1;
        for j in 0..height {
            // Виводимо пробіли зліва для вирівнювання
            (0..(num_triangles - i - 1 + j)).for_each(|_| print!(" "));
            (0..(width - 2 * j)).for_each(|_| print!("*"));
            println!();
        }
    }
    let trunk_width = 3;
    let trunk_height = 2;
    let trunk_padding = num_triangles - 2;
    for _ in 0..trunk_height {
        (0..trunk_padding).for_each(|_| print!(" "));
        (0..trunk_width).for_each(|_| print!("*"));
        println!();
    }
}
