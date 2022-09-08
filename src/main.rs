use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();
	let url = &args[1];
	let mut n = 3000;
	println!("Setting url {}", url);
	println!("Starting Scan Please wait");
	while n < 9999 {
//		println!("Counter {:?}",n);
		let num: String = n.to_string();

		if n < 10 {
			
			let response = reqwest::blocking::get(
			url.to_owned()+"000"+&num).unwrap().text().unwrap();
			if response.len() != 7888 {
				println!("{}",url.to_owned()+"000"+&num);
	                	println!("Length {:?}", response.len());
			}
		}

		if n < 100 && n > 9 {

                        let response = reqwest::blocking::get(
                        url.to_owned()+"00"+&num).unwrap().text().unwrap();

                        if response.len() != 7888 {
				println!("Length {:?}", response.len());
				println!("{}",url.to_owned()+"00"+&num);

			}
                }

                if n < 1000 && n > 99 {

                        let response = reqwest::blocking::get(
                        url.to_owned()+"0"+&num).unwrap().text().unwrap();
                        println!("{}",url.to_owned()+"0"+&num);
			if response.len() != 7888 {
                                println!("{}",url.to_owned()+"0"+&num);
                                println!("Length {:?}", response.len());
                        }
                }
                if n > 999 && n < 10000 {

                        let response = reqwest::blocking::get(
                        url.to_owned()+&num).unwrap().text().unwrap();
//                        println!("{}",url.to_owned()+&num);
                        if response.len() != 7888 {
                                println!("{}",url.to_owned()+&num);
                                println!("Length {:?}", response.len());
                        }
                }

		n += 1;
	}
}
