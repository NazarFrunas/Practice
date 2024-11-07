use rand::Rng;

fn gen_shipments(n: usize) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let mut shipments: Vec<u32> = (0..n).map(|_| rng.gen_range(1..100)).collect();
    let total: u32 = shipments.iter().sum();
    let target = total / n as u32;
    if total % n as u32 != 0 {
        let diff = total % n as u32;
        shipments[0] += diff;
    }

    shipments
}

fn main() {
    let shipments = gen_shipments(5);
    println!("Генерований вектор вантажів: {:?}", shipments);
}
