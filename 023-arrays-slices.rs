use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("1st element: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    let ys: [i32; 500] = [0; 500]; // all values set to 0

    println!("array occupies {} bytes", mem::size_of_val(&xs));

    println!("borrow whole thing as slice");
    analyze_slice(&xs);

    println!("borrow section of slice");
    analyze_slice(&xs[1..4]);
}
