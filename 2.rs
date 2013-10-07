fn sum_of_even_fibs (limit: int) -> int {
    let mut cntr = 0;
    let mut n0 = 0;
    let mut n1 = 1;
    let mut sum = 0;

    let mut n = 0;

    while n < limit {
        n = n1 + n0;
        n0 = n1;
        n1 = n;
        if (n % 2 == 0) {
            sum += n;
            println((fmt!("%d(%d)",n, sum)));
        } else {
            println(fmt!("%d",n));
        }
        cntr += 1;
    }
    return sum;
}

fn main() {
    println(fmt!("sum : %d\n",
                 sum_of_even_fibs(4000000)));

    }