use std::io;

// Elf Structure
// consists of a number identifying the order it was input, a vector of the calories of each of 
// their food items, and an optional total calorie tally
struct Elf { number: usize, food: Vec<u32>, calories: Option<u32> }

fn main() {
    // Store input as a vector of elf structures
    let mut elves = read_elves();

    // Calculate total calorie tallies for each elf
    for mut elf in elves.iter_mut() {
        // initalize the elf's calorie field
        elf.calories = Some(0);
        // add the calories of each food item
        for item in elf.food.iter() {
            elf.calories = Some(elf.calories.unwrap() + item);
        }
    }
    // sort the vector of elves by calorie totals
    elves.sort_by_key(|elf_key| elf_key.calories);

    //Print answers for AdventofCode
    println!("Elf w/ the most: elf #{}", &elves.last().unwrap().number);
    println!("Calories: {}", &elves.last().unwrap().calories.unwrap());
    println!("Calories of top 3 elves: {}", {
        // add last three elves's calorie totals
        &elves[elves.len() - 1].calories.unwrap() +
        &elves[elves.len() - 2].calories.unwrap() +
        &elves[elves.len() - 3].calories.unwrap()
    })
}

fn read_elves() -> Vec<Elf> {
    let mut elf_pool: Vec<Elf> = Vec::new();
    let mut elf_counter = 0;

    // Loop through each elf
    loop {
        // Read line
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        line.truncate(line.trim().len()); // println!("{}", line);
        // add elf to vector without calorie total
        elf_pool.push(Elf {number: elf_counter, food: Vec::new(), calories: None});

        // Loop through each elf's food
        loop {
            // Check for end of elf
            if line.is_empty() {
                elf_counter += 1;
                break;
            }

            // Add this line's calories to this elf's food vector
            elf_pool.last_mut().unwrap().food.push(line.parse().unwrap());

            // Read line
            line = String::new();
            // check for endoffile
            if io::stdin().read_line(&mut line).unwrap() == 0 { return elf_pool };
            line.truncate(line.trim().len()); // println!("{}", line);
        }
    }
}