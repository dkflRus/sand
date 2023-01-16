fn main(maxNum:u16) {
            println!("Initialisation...");
            use rand::Rng;
            let mut i;
            let mut iInt;
            let mut goalInt: u16;
            'all: loop {
                println!("Generating int...");
                goalInt = rand::thread_rng().gen_range(0..(maxNum+1));
                'singleGame: loop {
                    i = String::new();
                    std::io::stdin().read_line(&mut i).unwrap();
                    iInt = {
                        let (h, _) = i.split_at(i.len() - 1);
                        h
                    }
                    .parse::<u16>()
                    .unwrap_or(0);
                    if 0 >= iInt || iInt >= 1001 {
                        println!("Bad input!")
                    } else if iInt < goalInt {
                        println!("Too low!");
                    } else if iInt > goalInt {
                        println!("Too high!");
                    } else {
                        println!("Yay!");
                        println!("One more time? (Y/n)");
                        i = String::new();
                        std::io::stdin().read_line(&mut i);
                        match {
                            let (h, _) = i.split_at(1);
                            h
                        } {
                            "N" | "n" => {
                                break 'all;
                            }
                            _ => {
                                break 'singleGame;
                            }
                        }
                    }
                }
            }
        }