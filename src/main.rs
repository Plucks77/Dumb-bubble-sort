fn main() {
    let mut array = [5, 3, 12, 2, 22, 82, 5, -12];

    let mut sorted = false;

    while !sorted {
        for i in 0..array.len() - 1 {
            let current = array[i];
            let next = array[i + 1];

            if current > next {
                array[i + 1] = current;
                array[i] = next;
                break;
            }
            if i + 1 == array.len() - 1 {
                sorted = true
            }
        }
    }
    println!("Final array: {:?}", array)
}
