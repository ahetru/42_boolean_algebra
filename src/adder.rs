fn print_bits(name: &str,n: u32) {
	println!("{name}: {:032b}", n);
}

pub fn adder(mut a: u32, mut b: u32) ->u32 {
	while b!= 0 {
	let carry = (a & b) << 1;
	a = a ^ b;
	b = carry;
     }
     a
 }

 pub fn multiplier(mut a: u32, mut b: u32) -> u32 {
 	let mut result = 0;
  	while b != 0 {
   		if b & 1 != 0 {
     		result += a
     	}
      	a <<= 1;
       	b >>= 1;
   }
   result
 }
