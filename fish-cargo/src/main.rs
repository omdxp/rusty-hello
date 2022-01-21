fn main() {
    // let my_vector: Vec<i32> = Vec::new();
    let mut my_vector = vec![1, 2, 3, 4];

    println!("{}", my_vector[2]);

    my_vector.push(5);

    println!("{}", my_vector[4]);

    my_vector.remove(1); // remove the element at index 1 (2)

    for numver in my_vector.iter() {
        println!("{}", numver);
    }
}
