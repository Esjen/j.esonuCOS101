fn main() {
   let p:f64 = 210000.00;
   let r:f64 = 5.00;
   let n:f64 = 3.00;

   //computes the compound interest of depreciation
   let v= p * (1.0 - (r/100.0)).powf(n);
   println!("The value is {}", v);

 }  