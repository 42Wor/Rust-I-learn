use std::io;

fn main() {/* 
    let a=-1;
    let b="A";
    let c ='/';
    let d=true;
    let e:u32=11;
    println!("{}",a);
    print!("{}\n",b);
    print!("{}\n",c);
    print!("{}\n",d);
    println!("{}\n",e);
    let mut t:(i32,char,bool)=(1,'A',true);
    let  t=(1,'A',true,"A");
    let mut a=[1,1,3,345,3,33,3];
    let a=[1,1,3,345,3,33,3];
    println!("{}",a[2]);*/
    let mut i = String::new();
    io::stdin().read_line(&mut i).expect("faied");
    let n:i32=i.trim().parse().expect("e");
    if n%2!=0{
        print!("Odd");
    }
    else if n%2==0 {
        print!("Even")
    }
    else {
        print!("only number")
    }
    print!("\n{}",i);
}
