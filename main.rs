use std::env::args;

fn main() {

	let my_args: Vec<String> = args().collect();
	let ipv6_address = my_args[1].to_string();
	let group_strings: [&str; 8] = ipv6_address
		.split(".")
		.collect::<Vec<&str>>()
		.try_into()
		.unwrap_or_default();
	println!{"{:?}", group_strings};

}
