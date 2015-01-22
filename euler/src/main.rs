use std::collections::HashSet;
use std::collections::HashMap;
use std::iter::range_step;
use std::num::Int;
use std::num::Float;

// Helper functions

fn test() {
    let mut cancelled:HashSet<i64> = HashSet::new();
    let mut t:HashMap<int,int> = HashMap::new();
    let mut u:HashMap<int,int> = HashMap::new();
    t.insert(1,1);
    println!("{}", t);
    t.insert(1,3);
    t.insert(5,4);
    println!("{}", t);
    
    for (k,v) in t.iter() {
        println!("{} :: {}", k, v);
        match t.get(k) {
            Some(a) => {u.insert(*k, *a);},
            None => {println!("Nothing");}
        }
        u.insert(*k, *v);
    }


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

fn primes(n:int) -> Vec<int> {
    let mut cancelled:HashSet<int> = HashSet::new();
    let mut primes:Vec<int> = Vec::new();
    for i in range(2i, n+1i) {
        if !cancelled.contains(&i) {
            primes.push(i);
            for j in range_step(i, n+1, i) {
                cancelled.insert(j);
            }
        }
    }
    return primes;
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

fn factorise(n:int) -> Vec<int> {
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
    factors
}

// Solutions start here.

fn problem7(n:int) -> int{
    let p = primes(n);
    p[10000]
    // match p.pop() {
    //     Some(t) => t,
    //     None => 0
    //     }
    }

fn problem6(n:int) -> int{
    let n = n as f64;
    let sum_of_squares = (n*(n+1f64)*(2f64*n + 1f64))/6f64;
    let sum = (n * (n+1f64))/2f64;
    (sum.powi(2) - sum_of_squares ) as int

    }

fn problem5() -> int {
    let mut max_prime_powers:HashMap<int, int> = HashMap::new();
    for i in range(1, 21) {
        let mut prime_powers:HashMap<int,int> = HashMap::new();
        let factors = factorise(i);
        for j in factors.iter() {
            let n = match prime_powers.get(j) {
                Some(t) => *t + 1,
                None => 1
            };
            prime_powers.insert(*j, n);
        }

        for (k,v) in prime_powers.iter() {
            let n = match max_prime_powers.get(k) {
                Some(t) => { if t < v { *v } else { *t }},
                None => *v,
            };
            max_prime_powers.insert(*k, n);
        }
    }
    let mut prod:int = 1;
    for (k,v) in max_prime_powers.iter() {
        prod = prod * k.pow(*v as uint);
    }
    prod

}

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
    println!("Problem 5 : {}\n", problem5());
    println!("Problem 6 : {}\n", problem6(100));
    println!("Problem 7 : {}\n", problem7(200000));
    //600851475143


}
