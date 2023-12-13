fn main() {
    
    let _v = vec![10,20,30];
    //vector v owns the object in heap

    let v2 = vec![10,20,30]; //moves ownership to v2

    display(v2.clone());
    //v2 is moved to diplay and v2 is invalidated

    println!("In main {:?}", v2);
    //v2 is no longer usable here
}

fn display(v:Vec<i32>){
    println!("inside display {:?}", v);


}
