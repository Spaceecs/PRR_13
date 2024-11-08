use rand::Rng;
fn count_permutation(shipments: &Vec<u32>) -> usize {
    let n = shipments.len() as u32;
    let total_weight: u32 = shipments.iter().sum();

    if total_weight % n != 0 {
        return usize::MAX;
    }

    let target_weight = total_weight / n;
    let mut moves: usize = 0;

    for &weight in shipments {
        if weight > target_weight {
            moves += (weight - target_weight) as usize;
        }
    }

    moves
}
// Задача 2
fn gen_shipments(n: usize) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let mut shipments: Vec<u32> = (0..n).map(|_| rng.gen_range(1..=10)).collect();

    let total_weight: u32 = shipments.iter().sum();
    let remainder = total_weight % n as u32;
    if remainder != 0 {
        shipments[0] += (n as u32 - remainder);
    }

    shipments
}

#[test]
fn main() {
    let shipments = vec![1, 1, 1, 1, 6];
    let result = count_permutation(&shipments);
    println!("{}", result); // Виведе 4
    let n = 5;
    let shipments = gen_shipments(n);
    println!("{:?}", shipments);
}