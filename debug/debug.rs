struct UnPrintable(i32);

// The `derive` attribute automatically creates the implementation
// required to make this `struct` printable with `fmt::Debug`.
#[derive(Debug)]
struct DebugPrintable(i32);

#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a>{
    name: &'a str,
    age: u8
}

fn main() {
    let _test: UnPrintable = UnPrintable(123);
    let test_debug: DebugPrintable = DebugPrintable(123);
    // won't compile, test can't be compiled to string
    // println!("{:?} should be unprintable ", test);
    println!("{:?} should be debug printable", test_debug);

    println!("Now {:?} will print!", Structure(3));

    println!("Now {:?} will print!", Deep(Structure(7)));

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };
    
    // pretty printing
    println!("{:#?}", peter);
}