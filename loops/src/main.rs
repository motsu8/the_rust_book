fn main() {
    println!("---------------loop_fn");
    loop_fn();

    println!("---------------while_fn");
    while_fn();

    println!("---------------for_fn");
    for_fn();

    println!("---------------rev_for_fn");
    rev_for_fn();
}

fn rev_for_fn (){
    for element in (1..4).rev() {
        println!("{}!", element);
    }
    println!("LIFTOFF!!!")
}

fn for_fn (){
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("The value is: {}", element);
    }
}

fn while_fn(){
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }
    println!("LIFTOFF!!!");
}

fn loop_fn(){
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {}", count);
}
