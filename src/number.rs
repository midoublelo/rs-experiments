pub fn collatz(mut num: i32) {
    while num != 1 {
        if num % 2 == 0 {
            num = num / 2;
        } else {
            num = (num * 3) + 1;
        }
        println!("{}", num);
    }
}

pub fn find_biggest(num1: i32, num2: i32, num3: i32) {
    if num1 > num2 {
        if num1 > num3 {
            println!("The biggest number is {}", num1);
        } else {
            println!("The biggest number is {}", num3);
        }
    } else if num2 > num3 {
        println!("The biggest number is {}", num2);
    } else {
        println!("The biggest number is {}", num3);
    }
}
