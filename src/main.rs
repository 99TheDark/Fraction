use crate::frac::Fraction;

mod frac;

fn main() {
    {
        let f1 = Fraction::from(1.312837158976737); // 84935/64695
        let f2 = Fraction::from(-2.33333333333333); // -7/3
        let f3 = Fraction::from(-132.807726088146); // -9223895/69453
        let f4 = Fraction::from(0.0); // 0
        let f5 = Fraction::from(0.0 / 0.0); // nan
        let f6 = Fraction::from(6.0 / 0.0); // inf
        let f7 = Fraction::from(-19.0 / 0.0); // -inf
        let f8 = Fraction::from(5.0); // 5

        println!(
            "{}, {}, {}, {}, {}, {}, {}, {}",
            f1, f2, f3, f4, f5, f6, f7, f8
        );
    }

    {
        let f1 = Fraction::pos(5460, 104286); // Automatically simplifies fractions
        println!("{} = {}", f1, f1.value());

        let f2 = Fraction::new(true, 8, 9);

        println!("{}", f1 + f2); // 1618/1719
        println!("{}", Fraction::pos(51, 21) + Fraction::pos(5, 6)); // 137/42
        println!("{}", Fraction::neg(4, 7) * Fraction::pos(21, 5)); // -12/5
    }

    {
        // Final test with everything combined
        let b = -Fraction::pos(5, 4)
            * (Fraction::from(0.28571428571) - Fraction::pos(16, 42) / Fraction::new(true, 5, 2))
            + Fraction::from(3.82051282051)
            - Fraction::from(8.6);

        // Equivalent to:
        // -5/4 * (2/7 - (16/42) / (5/2)) + 149/39 - 43/5 = -643/130

        assert_eq!(
            (b * 2.5).value(), // = -643/130 * 5/2 = -643/52
            -12.365384615384615384615384615384615384615384615384615384615384615384615384615384615384615384615384615384615384615384615384615384615384615384615384615384615384615384615384615384615384615384615384615385
        );
    }
}
