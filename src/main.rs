// Define a trait for objects that can be printed
trait Printable {
    fn print(&self);
}

// Define a macro to generate the code for the PrintableObject enum and its implementation of the Printable trait
macro_rules! define_printable_object {
    // Match a list of enum variants with their associated data and printing logic
    ($($variant:ident { $($field:ident: $type:ty),* } => $print:block),*) => {
        // Generate the code for the PrintableObject enum
        enum PrintableObject {
            $($variant { $($field: $type),* }),*
        }
        
        // Generate the code for the implementation of the Printable trait for PrintableObject
        impl Printable for PrintableObject {
            fn print(&self) {
                match self {
                    // Generate a match arm for each variant of the PrintableObject enum
                    $(PrintableObject::$variant { $($field),* } => $print),*
                }
            }
        }
    }
}

// Use the define_printable_object macro to generate the code for the PrintableObject enum and its implementation of the Printable trait
define_printable_object!(
    RepeatedMessage { message: String, times: i32 } => {
        for _ in 0..*times {
            println!("{}", message);
        }
    },
    Message { message: String } => {
        println!("{}", message);
    }
);

fn main() {
    // Create a vector of printable objects
    let printables: Vec<PrintableObject> = vec![
        PrintableObject::RepeatedMessage { message: "foo".to_string(), times: 5 },
        PrintableObject::Message { message: "my name".to_string() },
    ];
    
    // Use an iterator to print each object using the fully qualified syntax
    printables.iter().for_each(Printable::print);
}