fn main() {
// // let x = 5;
// // println!("the value of x is{}", x);
// let tup  = (2, "hi", true);
// println!("{}", tup.1);

// let mut array = [1, "a", 2, 22];
// println!("{}", array[2]);

//tuples 
// let tup = (321, "hi", true);
// // println!("{}", tup.0);

// let(x, y, z) = tup;
// println!("{}",x);
// println!("{}",y);
// println!("{}",z);

// array:

// let array = [1,2,3];
// println!("{}", array[0]);

// let mut array1:[i32; 3] = [4,5,6];
// println!("{}",array1[0]);

// array1[0] = 10;
// println!("{}",array1[3]);

// //vector

// let mut nums = vec![1,2,3];

// nums.push(4);
// nums.pop();

// println!("{:?}", nums);


// let mut vec =  Vec::new(); //vec!
// vec.push("test");
// vec.push("String");
// vec.push("5");
// println!("{:?}", vec);

// vec.reverse();

// println!("{:?}", vec);

// let mut vect = Vec::<i32>::with_capacity(2);
// println!("{:?}",vect.capacity());

// let v: Vec<i32> = (0..5).collect();
// println!("{:?}", v);

// //slice
// let v: Vec<i32> = (0..5).collect();
// println!("{:?}",v);
// let sv: &[i32] = &v[2..4];          
// println!("{:?}", sv)

//String and &str

// let name = String::from("Nhuong");
// let course = "Rust".to_string();
// let new_name = name.replace("Nhuong", "Quy");
// println!("{:?}", name);
// println!("{:?}", new_name);
// println!("{:?}", course);

// //&str = "String slice " or "str"
// let str1 = "Hello";//&str
// // println!("{:?}", str1.bogus());
// let str2 = str1.to_string();
// let str3 = &str2;
// println!("{:?}", str1);
// println!("{:?}", str2);
// println!("{:?}", str3);


// //compare string  == != (does not equal)

// println!("{}","ONE".to_lowercase()=="one");

// //18. String literals

// let rust = "\x52\x75\x73\x74";
// println!("{:?}", rust)

//19. funtions

// print_phase("print my argument!");
println!("{:?}", gcd(2,5));
}

// fn print_phase(phrase: &str){
//     println!("{:?}", phrase);
// }

fn gcd(mut a: u64, mut b: u64)-> u64{
    while a!= 0{
        if a < b{
            let c = a;
            a = b;
            b = c;
        }
        a = a % b;
    }
    b
}