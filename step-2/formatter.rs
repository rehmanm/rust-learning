fn main(){

    let x = 10;
    println!("x = {}", x);

    println!("{0}, this is {1} and {1}, this is {0}", "Alice", "Bob");

    println!("{subject} {verb} {object}",
        subject = "the quick brown fox",
        object = "the brown fox",
        verb = "jumps"
    );

    //Special formatter ':'
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

    println!("{number:>width$}", number=1, width=6);

    println!("{number:>0width$}", number=1, width=6);

    println!("My Name is {0}, {1} {0}", "James", "Bond");

    #[allow(dead_code)]
    struct Structure(i32);

    //println!("This strut `{}` won't print...", Structure(3));

}