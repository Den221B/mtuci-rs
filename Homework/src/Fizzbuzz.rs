use std::ops::ControlFlow::Break;

fn main()
{
    let mut i = 1;
    loop
    {
        if i==101
        {
            break
        }
        else if i%15==0
        {
            println!("FizzBuzz");
        }
        else if i%3==0
        {
            println!("Fizz");
        }
        else if i%5==0
        {
            println!("Buzz");
        }
        else
        {
            println!("{}", i);
        }
        i += 1;
    }

}

