fn main() {
    let arr: [i32; 6] = [2, 4, 6, 8, 10, 12];

    'TEST: for number in arr {
        println!("Number out is: {}", number);

        for n in 1..10 {
            println!("Number in is: {:?}", n);
            if n == 8 {
                break;
            }
        }
    };
}