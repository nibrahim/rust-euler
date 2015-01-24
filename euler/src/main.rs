use std::collections::HashSet;
use std::collections::HashMap;
use std::iter::range_step;
use std::num::Int;
use std::num::Float;
use std::str::StrExt;
// use std::char::CharExt;

// Helper functions
#[cfg(not(test))]
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

    println!("Primes :: {}", primes(50));
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
    
    let t = "1234";
    // println!("{}", t.iter().fold(0i, |x, &y| {x + y}));
    let v = t.chars().filter_map(|x| {x.to_digit(10)}).fold(1, |x, y| {x * y});
    println!("{}", v);


    }

fn primes(n:int) -> Vec<int> {
    let mut cancelled:HashSet<int> = HashSet::new();
    let mut primes:Vec<int> = vec![2];
    for i in range_step(3i, n+1i, 2) {
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

fn problem10() -> int {
    let primes = primes(2000000);
    primes.iter().fold(0, |x, &y| {x + y})
    }

fn problem9() -> int {
    let m = 1000i;
    for c in range(1, 1001i) {
        for a in range(1, m-c) {
            let b = m-c-a;
            if a.pow(2 as uint) + b.pow(2 as uint) == c.pow(2 as uint) {
                return a*b*c;
            }
        }
    }
    0
}

fn problem8(n:int) -> uint{
    let input = "7316717653133062491922511967442657474235534919493496983520312774506326239578318016984801869478851843858615607891129494954595017379583319528532088055111254069874715852386305071569329096329522744304355766896648950445244523161731856403098711121722383113622298934233803081353362766142828064444866452387493035890729629049156044077239071381051585930796086670172427121883998797908792274921901699720888093776657273330010533678812202354218097512545405947522435258490771167055601360483958644670632441572215539753697817977846174064955149290862569321978468622482839722413756570560574902614079729686524145351004748216637048440319989000889524345065854122758866688116427171479924442928230863465674813919123162824586178664583591245665294765456828489128831426076900422421902267105562632111110937054421750694165896040807198403850962455444362981230987879927244284909188845801561660979191338754992005240636899125607176060588611646710940507754100225698315520005593572972571636269561882670428252483600823257530420752963450";
    let mut max = 0;
    for i in range(0i, 1000i)  {
        if (i+n) < 1000 {
            let v:Vec<uint> = input.slice(i as uint, (i+n) as uint)
                                   .chars()
                                   .filter_map(|x| {x.to_digit(10)}).collect();
            let prod = v.iter().fold(1, |x, &y| {x * y});
            if prod > max {
                max = prod;
            }
            // println!("{} {} {} {} {}", i, i+n, input.slice(i as uint, (i+n) as uint), v, prod);
        }
    }
    max
    }

fn problem7(n:int) -> int{
    let p = primes(n);
    p[10000]
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



#[cfg(not(test))]
fn main() {
    test();
    // Answers
    // Problem  1 : 233168
    // Problem  2 : 4613732
    // Problem  3 : 6857
    // Problem  4 : 906609
    // Problem  5 : 232792560
    // Problem  6 : 25164150
    // Problem  7 : 104743
    // Problem  8 : 23514624000
    // Problem  9 : 31875000
    // Problem 10 : 0
    let val = problem7(200000);
    println!("Problem 7 : {}  (104743)", val);
    assert_eq!(val, 104743);

    // println!(" Problem 10 {}",    problem10());
}

#[test]
fn test_problem1() {
    let val = problem1(1000);
    println!("Problem  1 : {}", val);
    assert_eq!(val, 233168);
}

#[test]
fn test_problem2() {
    let val = problem2(4000000);
    println!("Problem 2 : {}", val);
    assert_eq!(val, 4613732);
}

#[test]
fn test_problem3() {
    let val = problem3(600851475143);
    println!("Problem 3 : {}", val);
    assert_eq!(val, 6857);
}

#[test]
fn test_problem4() {
    let val = problem4();
    println!("Problem 4 : {}", val);
    assert_eq!(val, 906609);
}

#[test]
fn test_problem5() {
    let val = problem5();
    println!("Problem 5 : {}", val);
    assert_eq!(val, 232792560);
}

#[test]
fn test_problem6() {
    let val = problem6(100);
    println!("Problem 6 : {}", val);
    assert_eq!(val, 25164150);
}

#[test]
fn test_problem7() {
    let val = problem7(200000);
    println!("Problem 7 : {}", val);
    assert_eq!(val, 104743);
}

#[test]
fn test_problem8() {
    let val = problem8(13);
    println!("Problem 8 : {}", val);
    assert_eq!(val, 23514624000);
}

#[test]
fn test_problem9() {
    let val = problem9();
    println!("Problem 9 : {}", val);
    assert_eq!(val, 31875000);
}

#[test]
fn test_problem10() {
    let val = problem10();
    println!("Problem 10 : {}", val);
    assert_eq!(val, 142913828922);
}


