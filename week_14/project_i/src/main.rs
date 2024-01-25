//Rust program to access database structure and relations based on certain criteria

use std::io;

fn main() {
let mut input = String::new();
println!("NOTE: Job Title must either be administrator, project manager, employee, customer or vendor to access data.");
println!("\nIf you are a member of the management, what is your job title?");
io::stdin().read_line(&mut input).expect("Not a valid String");
let input = input.trim();

for _x in 0..input.len() {

if input == "administrator" {
println!("Database structure");
println!(" List of relations
 Schema |    Name    | Type  |  Owner
--------+------------+-------+----------
 public | customer   | table | postgres
 public | dataplan   | table | postgres
 public | department | table | postgres
 public | project    | table | postgres
 public | staff      | table | postgres");
}

else if input == "project manager" {
println!("\nProject table");
println!("pno | pname |                     pduration                      | project_managerid
          -----+-------+----------------------------------------------------+-------------------
            11 | A     | 9 Months                                           |               102
            22 | B     | 14 Months                                          |                97
            33 | C     | 16 Months                                          |               120
            44 | D     | 25 Months                                          |               108
            55 | A     | 9 Months                                           |               107");
}

else if input == "employee" {
println!("\nStaff table");
println!("staff_id |   staff_name   | dno | staff_sal | age |   mobile
     ----------+----------------+-----+-----------+-----+------------
           100 | Mustapha Ali   |   3 |    175000 |  32 | 8063285831
           107 | Alokwe Martin  |   7 |    380000 |  48 | 7090082812
            97 | Dankade Aminat |   5 |    550000 |  40 | 9023688832
           108 | Josiah Joshua  |   1 |    120000 |  30 | 8053189131
           102 | Mankinde Mary  |   2 |    450000 |  55 | 9023487830
           120 | Adeleke Jane   |   4 |    200000 |  38 | 7061045862
           122 | Osahon Mark    |   6 |    320000 |  44 | 8022289842
           104 | Kuti Lawal     |   1 |    750000 |  35 | 9145689842
           117 | Suleman Ajayi  |   3 |    800000 |  50 | 7030089981");
}

else if input == "customer" {
println!("\nCustomer table");
println!("      c_id |     c_name      | c_age |       c_email        |  c_mobile  | eid | data_id
         ------+-----------------+-------+----------------------+------------+-----+---------
           110 | Musta Karim     |    35 | m_karim@gmail.com    | 8055089112 | 102 |       5
           111 | Lilian Jaiya    |    43 | i_jaiye@gmail.com    | 8055185341 | 100 |       3
           112 | Arthur Musa     |    50 | a_musa@gmail.com     | 7055282813 | 107 |      10
           113 | Philip Akonjo   |    41 | p_akonjo@gmail.com   | 9052356772 | 100 |       2
           114 | Marylene Mapa   |    33 | m_mapa@gmail.com     | 8053333551 | 120 |       5
           115 | Oghenero Agor   |    50 | o_agor@gmail.com     | 7055566774 | 117 |      11
           116 | Adams Bree      |    33 | a_bree@gmail.com     | 8056765424 | 102 |       1
           117 | Okafor Mathias  |    45 | o_matthias@gmail.com | 8056763367 | 120 |      10
           118 | Samson Adeleke  |    65 | s_adeleke@gmail.com  | 7056774423 | 117 |      11
           119 | Lawal Tamire    |    35 | i_tamire@gmail.com   | 9052111101 | 107 |       5
           120 | James Job       |    44 | j_job@gmail.com      | 805969319  | 100 |       8
           121 | Matthew Jakande |    21 | m_jakande@gmail.com  | 7051232144 | 120 |       2
           122 | Jimila Adegboye |    20 | j_adegboye@gmail.com | 8054921923 | 107 |       5");
}

else if input == "vendor" {
println!("\nData-plan table");
println!("data_id | data_size | data_duration | data_price
         ---------+-----------+---------------+-------------
                1 | 350MB     | 2 days        | 200 naira
                2 | 1.8GB     | 14 days       | 500 naira
                3 | 3.9GB     | 30 days       | 1000 naira
                4 | 7.5GB     | 30 days       | 1500 naira
                5 | 9.2GB     | 30 days       | 2000 naira
                6 | 10.8GB    | 30 days       | 2500 naira
                7 | 14GB      | 30 days       | 3000 naira
                8 | 18GB      | 30 days       | 4000 naira
                9 | 24GB      | 30 days       | 5000 naira
               10 | 29.9GB    | 30 days       | 8000 naira
               11 | 50GB      | 30 days       | 10000 naira");
}

else {
    println!("\nThe Job Title specified has no corresponding table data.");
    break;
}
}

}



