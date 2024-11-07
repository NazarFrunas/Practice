fn print_pattern() {
    let m = 'm';
    let u = 'u';
    let x = 'x';
    let a = 'a';
    let s = 's';
    let l = 'l';
    let o = 'o';
    let n = 'n';

    println!("  {}{}{} {}", m, u, x, a);  // muxa
    println!("{}        {}", x, a);     // x        a
    println!("  ------");               // ------
    println!("    {}{}{} {}", s, l, o, n); // slon
}

fn main() {
    print_pattern();
}
