fn main() {
    
    let sum: u32 = vec![1, 2, 3].into_iter().map(|x| x * x).sum();
    println!("The result of (1*1)+(2*2)+(3*3) = {}", sum);
}
