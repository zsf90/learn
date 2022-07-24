macro_rules! create_function {
    ($func_name: ident) => {
        fn $func_name() {
            println!("You called {:?}()", stringify!($func_name));
        }
    };
}

macro_rules! print_result {
    ($expression: expr) => {
        println!("{:?} = {:?}", stringify!($expression), $expression)
    };
}

macro_rules! four {
    () => {
        1 + 3
    };
}

macro_rules! add_one {
    ($num: expr) => {
        println!("{:?}", $num + 1)
    };
}

macro_rules! num_add_one {
    ($num: expr) => {
        $num += 1;
        println!("{:#?}, {}", stringify!($num), $num);
    };
}

create_function!(foo);
create_function!(bar);

macro_rules! test {
    ($left: expr; and $right: expr) => {
        println!(
            "{:?} and {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left && $right
        )
    };
    ($left: expr; or $right: expr) => {
        println!(
            "{:?} and {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left || $right
        )
    };
}

macro_rules! find_min {
    ($x: expr) => ($x);
    ($x: expr, $($y: expr), +) => (
        std::cmp::min($x, find_min!($($y), +))
    )
}

fn main() {
    println!("Min: {}", find_min!(10, 20, 50));
}
