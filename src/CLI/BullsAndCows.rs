pub fn main(num: Option<u16>) {
    'all: loop {
        println!("Initialisation...");
        println!("I'll describe rules now\nBulls are nums that are correct and at correct place\nCows are numbers thst exists in the answer (no matter how many in the answer or in your input)\nBulls are not included in cows.\n(Example: in answer is 6726 and input is 6666, You'll recieve 2 bulls and 2 cows(Because of second and third 6))");
        use rand::Rng;
        let goalStr = num
            .unwrap_or(rand::thread_rng().gen_range(1001..10001))
            .to_string();

        let goalStrVec: Vec<char> = goalStr.chars().collect();

        let mut inputString;
        let mut inputCharVec: Vec<char>;
        let mut results = [0; 2];
        let mut usedCows: Vec<char>;
        let mut usedBulls: Vec<char>;
        println!("{} digits", goalStrVec.len());
        'singleGame: loop {
            results = [0; 2];
            inputString = String::new();
            std::io::stdin().read_line(&mut inputString);
            inputCharVec = {
                let (h, _) = inputString.split_at(inputString.len() - 1);
                h.chars().collect()
            };

            if inputCharVec.len() != goalStrVec.len()
                || inputString[..inputString.len() - 1].parse::<u16>().is_err()
            {
                if inputString == String::from("ans\n") {
                    println!("{}", goalStr);
                } else {
                    println!("Please write {} digits", goalStrVec.len());
                }
                continue 'singleGame;
            }

            for q in 0..inputCharVec.len() {
                if goalStrVec.contains(&inputCharVec[q]) {
                    if inputCharVec[q] == goalStrVec[q] {
                        results[1] += 1; //Correct place
                    } else {
                        results[0] += 1; //Correct
                    }
                }
            }

            if results[1] == goalStrVec.len() {
                println!("Yay!");
                println!("One more time? (Y/n)");
                inputString = String::new();
                std::io::stdin().read_line(&mut inputString);
                match {
                    let (h, _) = inputString.split_at(1);
                    h
                } {
                    "N" | "n" => {
                        break 'all;
                    }
                    _ => {
                        break 'singleGame;
                    }
                }
            } else {
                println!("Bulls:{} Cows:{}", results[1], results[0]);
            }
        }
    }
}