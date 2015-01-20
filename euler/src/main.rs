use std::collections::HashSet;
use std::iter::range_step;

// Helper functions

fn test() {
    let mut cancelled:HashSet<i64> = HashSet::new();
    cancelled.insert(34i64);
    println!("{}", cancelled);
    
    // let t:Vec<int> = vec![1,2,3,4];
    // for i in t.iter().rev() {
    //     println!("{}", i);
    //     }
    // for j in range(2i, 10i).rev() {
    //     println!("{}", j);
    // }
    // println!("11 {}", isprime(11));
    // println!("10 {}", isprime(100));

    // let mut i = 0i;
    // for i in range_step(1i, 100i, 2i) {
    //     println!("{}", i);
    //     }

    }


fn isprime(n:int) -> bool {
  if n % 2 == 0 {
      return n == 2;
  }
  if n % 3 == 0 {
      return n == 3;
  }
  if n % 5 == 0 {
      return n == 5;
  }
    let mut i = 7;
    while i*i < n {
        if n %i == 0 {
            return false
        }
        i = i +1;
    }
    return true
}

// Solutions start here.

fn problem4() -> int {
    let mut ret:int = 0;
    for i in range(100i, 1000i) {
        for j in range(100i, 1000i) {
            let front = (i *j).to_string();
            let back = front.as_slice().chars().rev().collect::<String>();
            if front == back {
                if ret <= i*j {
                    ret = i * j;
                }
            }
        }
    }
    return ret;
}


fn problem3(n:int) -> int {
    let mut factors:Vec<int> = Vec::new();
    let mut n = n;
    while n != 1 {
        for i in range(2i, n+1) {
            if isprime(i)  {
                if n%i == 0 {
                    factors.push(i);
                    n = n/i;
                    break;
                } 
            } 
        }
    }
    return match factors.pop() {
        None => 0,
        Some(t) => t
    };
}


fn problem2(lim:int) -> int {
    let mut acc = 0i;
    let mut a = 0i;
    let mut b = 1i;
    let mut tmp:int;
    while b < lim {
        acc += if b%2 == 0 { b } else { 0 };
        tmp = b;
        b = a + b;
        a = tmp;
    }
    return acc;
}

fn problem1(lim:int) -> int {
    let mut acc = 0i;
    for i in range(1i, lim) {
        acc += if i%3 == 0 || i%5 == 0 { i } else { 0 }
    }
    return acc;
}



fn main() {
    test();
    println!("------------------------------------------------------\n");
    println!("Problem 1 : {}\n", problem1(1000));
    println!("Problem 2 : {}\n", problem2(4000000));
    println!("Problem 3 : {}\n", problem3(600851475143));
    println!("Problem 4 : {}\n", problem4());
    //600851475143


}
