fn count_permutation(shipments: &Vec<u32>) -> usize {
    let total: u32 = shipments.iter().sum();
    let n = shipments.len();
    if total % n as u32 != 0 {
        return usize::MAX;
    }
    let target = total / n as u32;
    let mut moves = 0;
    let mut balance = 0;
    for &shipment in shipments.iter() {
        balance += shipment as i32 - target as i32;
        moves += balance.abs() as usize;
    }
    moves
}
fn main() {
    let shipments = vec![10, 20, 30, 40, 50];
    let moves = count_permutation(&shipments);

    if moves == usize::MAX {
        println!("Неможливо рівномірно розподілити вантаж!");
    } else {
        println!("Мінімальна кількість переносу вантажу: {}", moves);
    }
}
