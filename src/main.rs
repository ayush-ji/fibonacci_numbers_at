use std::env;

fn main() {

	let mut cli_args : Vec<String> = Vec::new();	

	for item in env::args() {
		cli_args.push(item);
	}

    let num = get_number(cli_args);
	println!("{}", get_f_num(num));
}

fn get_number(args : Vec<String>)  -> u32 {
		let mut digit : u32 = 0;
		for item in args {
			match item.trim().parse::<u32>() {
				Ok(num) =>  {
						digit = num;
						break;
					},
				Err(_) => continue,
			}
		}	
	digit
	}


fn get_f_num( digit : u32 ) -> u32 {
	let mut start = (0, 1);
	let mut finally : u32 = 0;
	if digit == start.0 {
		return 0	
	} else if digit == start.1 {
		return 1
	} else {
		for _x in 0..(digit-1) {
			finally = start.0 + start.1;
			start.0 = start.1;
			start.1 = finally;
		}
	}
	finally
}
