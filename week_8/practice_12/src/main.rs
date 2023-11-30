fn main() {
    
    // mutable array
    let mut colours = ["red","green","yellow","white"];

    println!("\nThe original array = {:?}", colours);

    //mutable slice
    let sliced_colours = &mut colours[1..3];

    println!("First value = {:?}", sliced_colours);

    //change the value of the original slice at the first index
    sliced_colours[1] = "purple";

    println!("Changed slice = {:?}", sliced_colours);
}
