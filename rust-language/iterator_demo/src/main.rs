fn main() {
    let v1 = vec![10, 2, 3, 4, 5, 20];
    let mut v1_iter = v1.iter();

    // for i in v1_iter {
    //     println!("{}", i);
    // }

    loop {
        match v1_iter.next() {
            None => break,
            Some(v) => println!("{}", v),
        }
    }
}
