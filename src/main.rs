use crate::frac::Fraction;

mod frac;

fn main() {
    {
        let f1 = Fraction::from(1.312837158976737); // 254802/194085
        let f2 = Fraction::from(-2.33333333333333); // -7/3
        let f3 = Fraction::from(-132.8077260881459); // -9223895/69453
        let f4 = Fraction::from(0.0); // 0
        let f5 = Fraction::from(0.0 / 0.0); // nan
        let f6 = Fraction::from(6.0 / 0.0); // inf
        let f7 = Fraction::from(-19.0 / 0.0); // -inf

        println!("{}, {}, {}, {}, {}, {}, {}", f1, f2, f3, f4, f5, f6, f7);
    }

    let f1 = Fraction::pos(5460, 104286); // Automatically simplifies fractions
    println!("{} = {}", f1, f1.value());

    let f2 = Fraction::new(true, 8, 9);

    println!("{}", f1 + f2);
    println!("{}", Fraction::pos(51, 21) + Fraction::pos(5, 6));
    println!("{}", Fraction::neg(4, 7) * Fraction::pos(21, 5));
}
