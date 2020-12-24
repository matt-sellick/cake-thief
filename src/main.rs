use std::{io, time::Instant};

// Puzzle 004 -- Cake Thief

// accepts a list of cake tuples and a duffel bag capacity, returns maximum value the bag can hold
fn max_duffel_bag_value(cake_tuples: &Vec<(u32, u32)>, capacity: u32) -> u32 {
    
    // cake and bag struct definitions
    #[derive(Debug)]
    struct Cake {
        weight: u32,
        value: u32,
    }
    impl Cake {
        fn new(info: (u32, u32)) -> Cake {
            Cake {
                weight: info.0,
                value: info.1,
            }
        }
    }
    struct Bag {
        space: u32, // remaining
        value: u32, // cumulative
        contents: String // descriptive
    }

    // create a cake-type structs vector using argument tuple data
    let mut cake_types: Vec<Cake> = Vec::new();
    for types in cake_tuples {
        cake_types.push(Cake::new(*types));
    }

    // sort the cake types by decreasing value:weight
    cake_types.sort_by_key(|k| (k.value / k.weight));
    cake_types.reverse();

    // create bag to store max values found
    let mut max_bag = Bag {
        space: capacity,
        value: 0,
        contents: String::new(),
    };

    // starting at each cake type, ...
    for start_idx in 0 .. cake_types.len() {

        // ... create a buffer bag ...
        let mut buf_bag = Bag {
            space: capacity,
            value: 0,
            contents: String::from("To maximize the value of your haul, the duffel bag can hold:\n"),
        };

        // ... test-fill duffel bags with as many of the densest cakes, by value, as possible
        for types in cake_types[start_idx ..].iter() {
            if types.weight <= buf_bag.space { // if there's space in the bag for at least one of the current type ...
                let quantity = (buf_bag.space - (buf_bag.space % types.weight)) / types.weight;
                buf_bag.value = buf_bag.value + (types.value * quantity); // update bag value
                buf_bag.space = buf_bag.space % types.weight; // update bag space
                
                // update "contents" text description
                buf_bag.contents.push_str(&quantity.to_string());
                buf_bag.contents.push_str(" of the { weight: ");
                buf_bag.contents.push_str(&types.weight.to_string());
                buf_bag.contents.push_str(", value: ");
                buf_bag.contents.push_str(&types.value.to_string());
                buf_bag.contents.push_str("} cake\n");
            }
        }

        // if the buffer bag value exceeds current max bag value, update max bag
        if buf_bag.value > max_bag.value {
            max_bag = buf_bag;
        }
    }

    // if nothing fits in the bag at all:
    if max_bag.value == 0 {
        max_bag.contents.push_str("The bag is too small for these cakes.");
    }

    // print the bag contents ...
    println!("{}", max_bag.contents);

    // ... and return the max bag value
    max_bag.value
}

// gets user number input
fn get_number() -> u32 {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).expect("Error getting input");
    let number: u32 = input_line.trim().parse().expect("Not a number"); // panics if invalid
    number
}

// gets user char input
fn get_char() -> char {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).expect("Error getting input");
    let character = match input_line.trim().parse() {
        Ok(ch) => ch,
        Err(_) => 'n', // returns 'n' if invalid
    };
    character
}

fn main() {
    println!("\nPuzzle #004: Cake Thief\n");

    // populate vector; .0 is weight and .1 is value
    let mut cake_list: Vec<(u32, u32)> = Vec::new();

    // choose cake types:
    println!("Choose a cake-type option:\n    1. (7, 160), (3, 90), (2, 15)\n    2. (100, 100), (30, 5), (65, 55)\n    3. Other");
    match get_number() {
        1 => {
            // at capacity 20, should return 555 shillings (6 x (3, 90) and 1 x (2, 15))
            cake_list.push((7, 160));
            cake_list.push((3, 90));
            cake_list.push((2, 15));
            println!("(Example capacity 20 should return 555 shillings)\n");
        },
        2 => {
            // at capacity 130, should return 100 shillings (2 x (65, 55))
            cake_list.push((100, 100));
            cake_list.push((30, 5));
            cake_list.push((65, 55));
            println!("(Example capacity 130 should return 110 shillings)\n");
        },
        _ => {
            println!("Enter your own value pairs (weight, value):");
            loop {
                cake_list.push((get_number(), get_number()));
                println!("Add another? (y/n): ");
                match get_char() {
                    'y' => continue,
                    'Y' => continue,
                    _ => break,
                }
            }
        },
    }

    // just to check ...
    println!("Queen Elizabeth's cakes, in (weight, value):\n{:?}\n", cake_list);
    
    // get capacity from user
    println!("What is your duffel bag's capacity, in kilos?: ");
    let capacity = get_number();
    println!("You entered: {} kilos\n", capacity);

    // set timer and call function
    let now = Instant::now();
    let value = max_duffel_bag_value(&cake_list, capacity);
    let elapsed = now.elapsed();

    // print result
    println!("The maximum value the duffel bag can hold is {} shillings.", value);
    println!("Elapsed time: {:?} \n", elapsed);
}