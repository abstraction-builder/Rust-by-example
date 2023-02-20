fn main() {
    println!("{} days", 31);

    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // named arguments
    println!("{subject} {verb} {object}",
            object="the lazy dog",
            subject="the quick brown fox",
            verb="jumps over"
            );

    // using format characters
    println!("Base 10 (decimal):        {:b}", 15213);
    println!("Base 2 (binary):          {:b}", 15213);
    println!("Base 8 (octal):           {:o}", 15213);
    println!("Base 16 (hexadecimal):    {:x}", 15213);
    println!("Base 16 (hexadecimal):    {:X}", 15213);

    println!("{number:>5}", number=1);
    println!("{number:0>5}", number=1);
    println!("{number:0>width$}", number=1, width=5);


    println!("My name is {0}, {1} {0}", "Bond", "James");
    // FIXME ^ Add the missing argument: "James"

    //#[allow(dead_code)]
    //struct Structure(i32);

    //println!("This struct {} won't print...", Structure(3));

    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");

    let pi = 3.141592;
    println!("Pi is roughtly {0:.3}", pi);
}