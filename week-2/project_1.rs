fn main() {
   let p:f64 = 520000000.0;
   let r:f64 = 10.0;
   let n:f64 = 5.0;

   //computes the compound interest
   let ci = p* (1.0 + (r/100.0)).powf(n) - p;
  use :: printIn!("Compound interest is {}", ci);

 }  