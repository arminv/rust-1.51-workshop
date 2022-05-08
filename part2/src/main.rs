struct City {
    description: String,
    residents: u64,

    // 👉 TODO add a field here for is_coastal: bool
    is_coastal: bool,
    //
    // 💡 HINT: this will cause other compiler errors.
    //    They will tell you what other changes need to happen!
}

fn new_city(residents: u64, is_coastal: bool) -> City {
    if is_coastal {
        City {
            description: format!("a *coastal* city of approximately {} residents", residents),
            residents,
            is_coastal,
        }
    } else {
        // panic!(
        //     "👉 TODO return a `City` described as a *non-coastal* city of approximately {} residents"
        // );
        City {
            description: format!("a *non-coastal* city of approximately {} residents", residents),
            residents,
            is_coastal,
        }
    }
}

fn main() {
    let rustville: City = new_city(855339, false);

    println!("This city can be described as: {}", rustville.description);

    if rustville.is_coastal {
        println!("It is a coastal city.");
    } else {
        println!("It is not a coastal city.");
    }
}
