fn complex_function(x: i32, y: i32) -> i32 {
    if x > 0 {
        if y > 0 {
            if x > y {
                if x > 100 {
                    if y > 50 {
                        println!("Branch 1");
                        x + y
                    } else {
                        println!("Branch 2");
                        x - y
                    }
                } else {
                    println!("Branch 3");
                    x * y
                }
            } else {
                println!("Branch 4");
                x / y
            }
        } else {
            println!("Branch 5");
            -y
        }
    } else {
        println!("Branch 6");
        -x
    }
}
