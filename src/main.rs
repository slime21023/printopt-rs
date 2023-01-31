use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let args_slice = &args[1..];
    let options = args_slice.iter()
        .filter(|item| item.starts_with("-"));

    for (index, opt) in options.enumerate() {
        println!("option {}:  {}", index, opt);
    }

    let parameters =  args_slice.iter()
        .filter(|item| !item.starts_with("-"));

    for (index, param) in parameters.enumerate() {
        println!("parameter {}:  {}", index, param);
    }
}
