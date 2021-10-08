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

			let itax = arr[i].prc * 0.05;
			prc = prc + itax ;
		/*	println!("> {}", itax);
			let rond = (arr[i].prc * 100.0).round() / 100.0;
			println!("> {}", rond);
			
		*/
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

/*
Input   1:
>   1   book   at   12.49        
>   1   music   CD   at   14.99        
>   1   chocolate   bar   at   0.85    

Input   2:    
>   1   imported   box   of   chocolates   at   10.00        
>   1   imported   bottle   of   perfume   at   47.50        
  
Input   3:    
>   1   imported   bottle   of   perfume   at   27.99        
>   1   bottle   of   perfume   at   18.99        
>   1   packet   of   headache   pills   at   9.75        
>   1   box   of   imported   chocolates   at   11.25   

Output   1:    
>   1   book:   12.49        
>   1   music   CD:   16.49        
>   1   chocolate   bar:   0.85        
>   Sales   Taxes:   1.50        
>   Total:   29.83        
  
Output   2:    
>   1   imported   box   of   chocolates:   10.50        
>   1   imported   bottle   of   perfume:   54.65        
>   Sales   Taxes:   7.65        
>   Total:   65.15    
  
Output   3:    
>   1   imported   bottle   of   perfume:   32.19        
>   1   bottle   of   perfume:   20.89        
>   1   packet   of   headache   pills:   9.75        
>   1   imported   box   of   chocolates:   11.85        
>   Sales   Taxes:   6.70        
>   Total:   74.68  
*/

