// strings1.rs
//
// Make me compile without changing the function signature!
//
// Execute `rustlings hint strings1` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);

    let mut heap = vec![123];
    heap.push(5);
    heap.push(9);
    heap.push(2);
    heap.push(1);

    // [1,5)
    for i in 1..5 {
        println!("{i}");
    }

    for x in &heap {
        print!("{x} ")
    }

    let fifth = heap.get(6);
    match fifth {
        Some(fifth) => println!(" fifth: {fifth}"),
        None => println!("There is no fifth element"),
    }


}

fn current_favorite_color() -> String {
    String::from("blud")
    // "blue".to_string()
}
