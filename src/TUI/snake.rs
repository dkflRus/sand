pub fn main(X: u16, Y: u16, wallHack: bool,pause:u32) {
    use rand::Rng;
    use std::time::{Duration, Instant};

    extern crate termion;

    use termion::raw::IntoRawMode;
    use termion::async_stdin;
    use std::io::{Read, Write, stdout};
    use std::thread;

    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();

    #[derive(Eq, PartialEq)]
    struct Point {
        x: u16,
        y: u16,
    }

    let mut path: Vec<Point> = Vec::new();
    let mut rotation: (bool, bool) = (false, false); //00-up, other counterclockwise
    let mut apple = Point {
        x: rand::thread_rng().gen_range(1..X - 1),
        y: rand::thread_rng().gen_range(1..Y - 1),
    };
    let mut Paused=false;

    fn edit(curr: u16, lim: u16,currWallHack:bool) -> u16 {
        if curr <= 0 {
            if currWallHack{lim - 2}
            else{panic!()}
        } else if curr >= lim - 1 {
            if currWallHack{1}else{panic!()}
        } else {
            curr
        }
    }

    'singleGame: loop {
        path.push(Point { x: X / 2, y: Y / 2 });
        while path.contains(&apple) {
            apple = Point {
                x: rand::thread_rng().gen_range(0..X),
                y: rand::thread_rng().gen_range(0..Y),
            };
        }

        loop {//'tillApple:
            
            //render
            write!(stdout,
                "{}{}",
                termion::clear::All,
                termion::cursor::Goto(1, 1))
                 .unwrap();
            for currentY in 0..Y {
                for currentX in 0..X {
                    let currentDot = Point {
                        x: currentX,
                        y: currentY,
                    };
                    print!("{}",{
                        if path.contains(&currentDot) {
                            "#"
                        } else if apple == currentDot {
                            "@"
                        } else if (currentX == 0 || currentX == X - 1)
                            || (currentY == 0 || currentY == Y - 1)
                        {
                            if wallHack {
                                "."
                            } else {
                                "X"
                            }
                        } else {
                            " "
                        }
                    })
                }
                print!("\n\r")
            }
            println!("len:{}",path.len());
            


            let now = Instant::now();
            'press: loop {
                let b=stdin.next();
                if let Some(Ok(key)) = b{
                    if key==b'q'{panic!("exit")}
                    if key==b'p'{
                        if Paused{Paused=false;}
                        else{
                            write!(stdout,
                            "{}{}",
                            termion::clear::All,
                            termion::cursor::Goto((X-(6/2))/2, Y/2-1 ))
                            .unwrap();
                            // print!("\n\r");
                            println!("paused");
                            Paused=true;
                            }
                        // panic!();
                        thread::sleep(Duration::from_millis(100));

                    }
                    if Paused{continue 'press;}
                    if rotation.1 {
                        if key==b'w'{
                            rotation = (false, false);
                            break 'press;
                        } else if key==b's'{
                            rotation = (true, false);
                            break 'press;
                        }
                    } else {
                        if key==b'a'{
                            rotation = (false, true);
                            break 'press;
                        } else if key==b'd'{
                            rotation = (true, true);
                            break 'press;
                        }
                    }
                }

                if !Paused && Instant::now() >= now + Duration::new(0, pause) {
                    break 'press;
                }
            }

            println!("{:?}", rotation);
            path.push(Point {
                x: ((path[path.len() - 1].x as i16) + {
                    if rotation.1 {
                        if rotation.0 {
                            1
                        } else {
                            -1
                        }
                    } else {
                        0
                    }
                } as i16) as u16,
                y: {
                    (path[path.len() - 1].y as i16) + {
                        if !rotation.1 {
                            if rotation.0 {
                                println!("3");
                                1
                            } else {
                                println!("4");
                                -1
                            }
                        } else {
                            0
                        }
                    } as i16
                } as u16,
            });

            let h = path.remove(path.len() - 1); //(path[path.len() - 1].y - 1) % (Y as u16 - 2) + 1
                path.push(Point {
                    x: edit(h.x, X,wallHack),
                    y: edit(h.y, Y,wallHack),
                });

            if path[path.len() - 1] == apple {
                while path.contains(&apple) {
                    apple = Point {
                        x: rand::thread_rng().gen_range(1..X - 1),
                        y: rand::thread_rng().gen_range(1..Y - 1),
                    };
                }
            } else if path[..path.len() - 1].contains(&path[path.len() - 1]) {
                break 'singleGame;
            }
            else{path.remove(0);}

        }
    }
}
