fn sum (x: &int) -> &int {
    x
}
    let q = 10;
    let w = 20;
    let p1;
    {
        let p2 = 50;
        p1 = &p2;
        println!(p1);
    }
    let a = sum(&w);
    println!(a);
