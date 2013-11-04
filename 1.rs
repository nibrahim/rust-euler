fn main() {
    let mut cntr = 1;
    let mut sum = 0;
    while (cntr < 1000) {
        if (cntr % 3 == 0 || cntr % 5 == 0) {
            sum += cntr;
        }
        cntr+=1;
    }
    println(format!("{}\n", sum));
}
