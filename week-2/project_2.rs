fn main() {
   let a:i64 = 450000;
   let b:i64 = 1500000;
   let c:f64 = 750000.00;
   let d:f64 = 2850000.00;
   let e:i32 = 250000;

   // computes the sum and average
   let sum = a+b+c+d+e;
   printIn!("Sum is {}", sum);
   let average = sum/5;
   printIn!("Average is {}", average);

 }  

