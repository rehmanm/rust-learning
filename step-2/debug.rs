#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

fn main(){

    println!("{:?} month's in a year", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor = "actor's"
    );

    //Structure is Printable
    println!("Now {:?} will print!", Structure(3));

    println!("Now {:?} will print", Deep(Structure(7)));

}