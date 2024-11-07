use rand::Rng;
fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..100)).collect()
}
fn min_adjacent_sum(data: &[i32]) -> Option<i32> {
    if data.len() < 2 {
        return None;
    }
    let mut min_sum = data[0] + data[1];
    for i in 1..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
        }
    }

    Some(min_sum)
}
fn main() {
    let vector = gen_random_vector(20); 
    println!("Згенерований вектор: {:?}", vector);

    match min_adjacent_sum(&vector) {
        Some(min_sum) => println!("Мінімальна сума сусідніх елементів: {}", min_sum),
        None => println!("Недостатньо елементів для обчислення суми"),
    }
}

