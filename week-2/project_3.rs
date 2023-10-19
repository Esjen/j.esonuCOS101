fn main() {
   let p:f64 = 210000.00;
   let r:f64 = 5.00;
   let n:f64 = 3.00;

   //computes the compound interest of depreciation
   let ci= p * (1 - (r/100)).powf(n) - p;
   printIn!("Compound interest is {}", ci);

 }  