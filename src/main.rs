// fn main() {
//     let r;
//     {
//         let x = 5;
//         r = x;
//     }
//     println!("{}", r);
// }

//Result shows that refrence of x was assigned to r, but refrence will remain valid as long as the scope of the variable is valid within the inner curly brace, after
// inner curly brace is finished the scope ends so compiler gives the error " borrowed value does not live long"

// PS E:\12 - PIAIC\IOT\Q2\IOTAssignments\PIAIC115352-Assignment3> cargo run
//    Compiling PIAIC115352-Assignment3 v0.1.0 (E:\12 - PIAIC\IOT\Q2\IOTAssignments\PIAIC115352-Assignment3)
// error[E0597]: `x` does not live long enough
//  --> src\main.rs:5:7
//   |
// 5 |     r=&x;
//   |       ^^ borrowed value does not live long enough
// 6 | }
//   | - `x` dropped here while still borrowed
// 7 |     println!("{}",r);
//   |                   - borrow later used here

// error: aborting due to previous error

// For more information about this error, try `rustc --explain E0597`.
// error: could not compile `PIAIC115352-Assignment3`.

// To learn more, run the command again with --verbose.
// PS E:\12 - PIAIC\IOT\Q2\IOTAssignments\PIAIC115352-Assignment3>

// Question 2
use std::fmt;
struct Students<'a, 'b, 'c, 'd> {
    name: &'a str,
    age: &'b str,
    education: &'c str,
    timing: &'d str,
}

impl Students<'_, '_, '_, '_> {
    fn get_age(&self) -> &str {
        &self.age
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_timing(&self) -> &str {
        &self.timing
    }

    fn get_education(&self) -> &str {
        &self.education
    }
}

impl fmt::Display for Students <'_, '_, '_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{},{},{})", self.name, self.age, self.education, self.timing)
    }
}

fn main() {
    let m1 = Students {
        name: "Shaukat",
        age: "35",
        education: "BE",
        timing: "evening",
    };

    let m2 = Students {
        name: "Hussain",
        age: "28",
        education: "BE",
        timing: "Morning",
    };

    println!("{}", m1.get_name());
    println!("{}", m1.get_age());
    println!("{}", m1.get_education());
    println!("{}", m1.get_timing());

    println!("{}",m1);
    println!("{}",m2);
}
