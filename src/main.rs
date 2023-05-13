use std::env::{Args, args};
use std::process::ExitCode;
//use std::process::exit;


fn operate(operator: String, f_first: f32, f_second: f32) -> f32
{
    match operator.as_str(){
        "+" => return f_first + f_second,
        "-" => return f_first - f_second,
        "*" => return f_first * f_second,
        "/" => return f_first / f_second,
        "%" => return f_first % f_second,
        "^" => return f_first.powf(f_second),
        _ => {
            //eprintln!("Wrong operator!\nUse one of these [+, -, *, /, %, ^]!");
            panic!("\n\t\tWrong operator!\n\t\tUse one of these [+, -, *, /, %, ^]!\n");
            //exit(1);
        }
    }
}

fn main() -> ExitCode {
    let mut arg: Args = args();

    if arg.len() != 4
    {
        //eprintln!("Wrong number of arguments!\nProgramm takes 3 arguments!");
        panic!("\n\t\tWrong number of arguments!\n\t\tProgramm takes 3 arguments!\n");
//        return ExitCode::from(1);
    }

    let first: String = arg.nth(1).unwrap();
    let op: String = arg.nth(0).unwrap();
    let second: String = arg.nth(0).unwrap();

 //   println!("hello: {} {} {}", first, op, second);

    let f_first: f32 = first.parse::<f32>().unwrap();
    let f_second: f32 = second.parse::<f32>().unwrap();

 //   println!("nums: {} {}", f_first, f_second);
 
    println!("result: {}", operate(op, f_first, f_second));
    return ExitCode::from(0);
}

