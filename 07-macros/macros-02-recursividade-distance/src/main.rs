macro_rules! distance {
    ($a: ident, $b: expr, $c: expr) => {
        let $a = {
            if $c > $b {
                $c - $b
            } else {
                $b - $c
            }
        };
    }
}
// metavariaveis começam co $ e são trocados por valores etc

fn main() {
    distance!(x, 3, 5);
    distance!(y, 5, 3);
    println!("{}, {}", x, y);

    distance!(a, 42 + 54, 112 + 33);
    distance!(b, 99 - 12, 1024 + 1);
    println!("{}, {}", a, b)
}
