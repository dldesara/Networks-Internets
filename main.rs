use std::env::args;
use std::vec::Vec;
use std::process;


// input is target ip in cidr notation
// output is 7 subnet values


// accepting command line arguments and saving values to variables

fn main() {

	for (i,arg) in args().enumerate(){
		println!("Arg {i}: {arg}");
		
		if i == 1{
			calculate_subnet(arg);
	}
}
}
fn calculate_subnet(target_subnet: String){

	
	let mut octets = target_subnet.split("/");
	let target_ip = octets.next().unwrap_or_default();
	let cidr = octets.next().unwrap_or_default();
	
	
	let mut octet_strings = target_ip.split(".");
	let mut groups = octet_strings
		.map(|octet| octet.parse::<u8>().unwrap_or_default())
        .collect::<Vec<_>>();
	println!("Target IP: {target_ip}");
	println!("CIDR: {cidr}");
	

	let mut column1: Vec<u8> = vec![128, 64, 32, 16, 8, 4, 2, 1];
    let mut column2: Vec<u8> = vec![128, 192, 224, 240, 248, 252, 254, 255];
    let mut column3: Vec<u8> = vec![25, 26, 27, 28, 29, 30, 31, 32];
	
	let mut network_id = 0;
	let mut next_one = 0;
	let mut broadcast_ip = 0;
	let mut first_host = 0;
	let mut last_host = 0;
	let mut num_ip = 0;
	let mut mask = 0;
	
	for i in 0..(column1.len() - 1){
		let cur = column1[i];
		let nx = column1[i + 1];
		if groups[3] < cur && groups[3] > nx {
				network_id = nx;
				next_one = cur;
				broadcast_ip = next_one - 1;
				first_host = network_id + 1;
				last_host = broadcast_ip-1;
				
				
		}
					

	}
println!("Network ID:{:08}", format!("{}.{}.{}.{}", groups[0], groups[1], groups[2], network_id));
println!("First host:{:08}", format!("{}.{}.{}.{}", groups[0], groups[1], groups[2], first_host));
println!("Last Host: {:08}", format!("{}.{}.{}.{}", groups[0], groups[1], groups[2], last_host));
println!("Next Network {:08}", format!("{}.{}.{}.{}", groups[0], groups[1], groups[2], next_one));
println!("Broadcast IP: {:08}", format!("{}.{}.{}.{}", groups[0], groups[1], groups[2], broadcast_ip));


	//println!("# of IPs: {num}");
	//println!("CIDR mask: 255.255.255.{mask}");

	//let num: 
	//let mask:

let cidr_num = cidr.parse::<u8>().unwrap();
	for j in 0..(column3.len() - 1){
		if cidr_num == column3[j]{
				num_ip = column1[j];
				
		}
					

	}


println!("Number of IPs {}", num_ip);

	for k in 0..(column2.len() - 1){
		if cidr_num == column3[k]{
				mask = column2[k];
						
		}
					

	}
println!("CIDR Mask: {:08}", format!("255.255.255.{}", mask));
	
}
