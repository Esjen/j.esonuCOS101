fn main() {
   let qt:f64 = 2.00;
   let t:f64 = 450000.00;
   println!("The amount for TOSHIBA is {} and the quantity is {}",qt,t);

   let qm:f64 = 1.00;
   let m:f64 = 1500000.00;
   println!("The amount for MAC is {} and the quantity is {}",qm,m );

   let qh:f64 = 3.00;
   let h:f64 = 750000.00;
   println!("The amount for HP is {} and the quantity is {}",qh,h );

   let qd:f64 = 3.00;
   let d:f64 = 2850000.00;
   println!("The amount for DELL is {} and the quantity is {}",qd,d );

   let qa:f64 = 1.00;
   let a:f64 = 250000.00;
   println!("The amount for ACER is {} and the quantity is {}",qa,a );

   let sum = (qt*t)+(qm*m)+(qh*h)+(qd*d)+(qa*a);
   println!("The Sum is {}",sum );
   
   let n=qt+qm+qh+qd+qa;
   let avg = sum/n;
   println!("The Average is {}",avg );

}