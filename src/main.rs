fn main() {
    let mut numbers = vec![5, 6, 4, 2, 3, 2, 2, 1, 7, 9, 8];
   numbers.sort();
    let odd = odd_count(numbers.clone());
    let even=even_count(numbers.clone());
    println!("Sorted numbers is {:?},{:?}, even: {:?}",numbers, odd,even);
}
fn odd_count(odd_count: Vec<u8>) -> f64 {
    let u = odd_count.len();
    let odd_median = (u + 1) / 2;
    odd_median as f64
}
fn even_count(even_count: Vec<u8>) -> f64 {
    let q = even_count.len();
    let even_median = ((q / 2) + ((q / 2) + 1)) / 2;
    even_median as f64
}
