enum CitySize {
    Town,       // approximate residents: 1_000
    City,       // approximate residents: 10_000
    Metropolis, // approximate residents: 1_000_000
}

struct City {
    description: String,
    residents: u64,
    is_coastal: bool,
}

impl City {
    // NOTE: instead of `City`, the return type could also have been `Self`:
    fn new(city_size: CitySize, is_coastal: bool) -> City {
        // NOTE: tuple destructuring to get `description` and `residents`:
        let (description, residents) = match city_size {
            CitySize::Town => {
                let residents = 1_000;

                // NOTE: returning a tuple:
                (
                    format!("a *town* of approximately {} residents", residents),
                    residents,
                )
            }
            CitySize::City => {
                let residents = 10_000;

                (
                    format!("a *town* of approximately {} residents", residents),
                    residents,
                )
            }
            CitySize::Metropolis => {
                let residents = 1_000_000;

                (
                    format!("a *town* of approximately {} residents", residents),
                    residents,
                )
            }
            // 👉 TODO Handle the other CitySize variants individually,
            //    in a similar way to how *town* is handled here
            _ => {
                let residents = 1_000;

                (
                    format!(
                        "an *unknown-size city* of approximately {} residents",
                        residents
                    ),
                    residents,
                )
            }
        };

        City {
            description,
            residents,
            is_coastal,
        }
    }
}

fn main() {
    // 👉 TODO Use City::new() to create a Metropolis-sized city here
    // (i.e. replace the commented out code below which is 'hardcoded' version of creating a struct):
    // let rustville = City {
    //     description: String::new(),
    //     residents: 0,
    // };
    let rustville = City::new(CitySize::Metropolis, true);

    println!("This city is {}", rustville.description);

    if rustville.residents > 100_000 {
        println!("Wow!");
    }
}
