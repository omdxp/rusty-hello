fn main() {
    let numbers = [1, 2, 3, 4, 5];
    // let numbers: [i32; 5] = [1, 2, 3, 4, 5];

    numbers[0]; // 1
    numbers[4]; // 5

    for n in numbers.iter() {
        println!("{}", n)
    }

    for i in 0..numbers.len() {
        println!("{}", numbers[i])
    }

    let numbers2 = [2; 400];
    for n in numbers2.iter() {
        println!("{}", n)
    }
}
