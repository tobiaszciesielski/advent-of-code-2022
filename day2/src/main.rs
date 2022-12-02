use std::fs;

fn main() {
    let game = fs::read_to_string("src/input.txt").expect("Should have been able to read the file");
    let rounds = game.lines();

    let mut points = 0;

    for round in rounds {
        let mut round = round.split_whitespace();

        let enemy = round.next().expect("String");
        let me = round.next().expect("String");

        if me == "X" {
            points += 1;

            if enemy == "A" {
                points += 3
            } else if enemy == "B" {
                // lose
            } else if enemy == "C" {
                points += 6
            }
        } else if me == "Y" {
            points += 2;

            if enemy == "A" {
                points += 6
            } else if enemy == "B" {
                points += 3
            } else if enemy == "C" {
                // lose
            }
        } else if me == "Z" {
            points += 3;

            if enemy == "A" {
                // lose
            } else if enemy == "B" {
                points += 6
            } else if enemy == "C" {
                points += 3
            }
        }
    }

    print!("{points}")
}
