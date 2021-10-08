#[derive(Debug)]

// Data Structure of Inputs
struct Inputs {
	qnt: i32,
	txt: String,
	prc: f32,
}

fn main() {

	// Test Input 1
	let t1: [Inputs; 3] = [
		Inputs {qnt: 1, txt: "book".to_string(), prc: 12.49},
		Inputs {qnt: 1, txt: "music CD".to_string(), prc: 14.99},
		Inputs {qnt: 1, txt: "chocolate bar".to_string(), prc: 0.85}
	];

	// Test Input 2
	let t2: [Inputs; 2] = [
		Inputs {qnt: 1, txt: "imported box of chocolates".to_string(), prc: 10.00},
		Inputs {qnt: 1, txt: "imported bottle of perfume".to_string(), prc: 47.50}
	];

	// Test Input 3
	let t3: [Inputs; 4] = [
		Inputs {qnt: 1, txt: "imported bottle of perfume".to_string(), prc: 27.99},
		Inputs {qnt: 1, txt: "bottle of perfume".to_string(), prc: 18.99},
		Inputs {qnt: 1, txt: "packet of headache pills".to_string(), prc: 9.75},
		Inputs {qnt: 1, txt: "box of imported chocolates".to_string(), prc: 11.25}
	];

	println!("{}", "Output 1:");
	build_invoice(&t1);
	println!("\n{}", "Output 2:");
	build_invoice(&t2);
	println!("\n{}", "Output 3:");
	build_invoice(&t3);

}


fn build_invoice (arr: &[Inputs]) {

	// Word Filter for non taxable products
	let tax_filter: [String;7] = ["book".to_string(), 
				"food".to_string(), 
				"medical".to_string(), 
				"pill".to_string(), 
				"chocolate".to_string(), 
				"books".to_string(), 
				"pills".to_string()];

	// Price, Tax and Sum as float
	let mut prc: f32 = 0.0;
	let mut tax: f32 = 0.0;
	let mut sum: f32 = 0.0;

	// For every element of the Test input
	for i in 0..arr.len() {
		
		// compare the String txt with every entry of Word Filter list
		'taxable: for j in 0..tax_filter.len() {

			// if the filter matches: break loop
			if arr[i].txt.contains(&tax_filter[j]) {
				prc = arr[i].prc;
				break 'taxable;
			} else {
				// else add Sales tax of 10%
				prc = arr[i].prc + 0.1*(arr[i].prc) ;
			}
		
		}
		
		// if the String txt is imported then ...
		if arr[i].txt.contains("import") {
				
			// ... +5% on sales price
			let mut itax = arr[i].prc * 0.05;
			// round to nearest .05
			// Fix because .56.. was always rounded down
			itax = ( (itax * 10.00 ).ceil() ) / 10.00; 
			itax = (itax / 0.05).round() * 0.05;
			prc = prc + itax ;
		}

		// The Tax is the substract of Brutto - Netto
		tax = tax + prc - arr[i].prc;
		sum = sum + prc;

		// Print every aricle of the test list
		println!(">  {}  {}:   {:.2} ", arr[i].qnt, arr[i].txt, prc);
		
		// only at the end print the Tax and total
		if i == arr.len() -1 {
			println!(">  Sales Tax:   {:.2} ", tax);
			println!(">  Total:   {:.2} ", sum);
		}
	}

}

