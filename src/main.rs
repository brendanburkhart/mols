mod latin_square;
mod permutations;

fn main() {
    for n in 1..7 {
        let generator = latin_square::Generator::new(n);

        let mut count = 0;
        for _square in generator {
            //println!("{}", square);
            count += 1;
        }

        println!("Number of {}x{} Latin Squares: {}", n, n, count);
    }
}
