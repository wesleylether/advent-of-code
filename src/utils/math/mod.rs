#[derive(Debug)]
pub struct Math {}

impl Math {
	pub fn prime_factors(mut n: usize) -> Vec<usize> {
		let mut factors: Vec<usize> = Vec::new();

		// Divide in 2 before uneven
		while n % 2 == 0 {
			factors.push(2);
			n /= 2
		}

		// n is now uneven, next step (i = 3, 5, 7, ...)
		let mut i = 3;
		while i * i <= n {
			while n % i == 0 {
				factors.push(i);
				n /= i;
			}
			i += 2;
		}

		// add last number if n > 2
		if n > 2 {
			factors.push(n);
		}

		factors
	}
}
