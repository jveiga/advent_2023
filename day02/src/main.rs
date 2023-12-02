use anyhow::{bail, Result};

fn main() {
    let input = include_str!("../input1.txt");
    // let input = include_str!("../ex.txt");
    let games: Vec<_> = input.lines().map(parse_line).collect();
    let games: Vec<_> = games.into_iter().filter_map(Result::ok).collect();
    dbg!(games.len());
    println!(
        "{}",
        games
            .iter()
            .map(|game| {
                println!(
                    "{:?} => {}",
                    game,
                    game.red <= 12 && game.green <= 13 && game.blue <= 14
                );
                game.id
            })
            .sum::<u32>()
    );
    let input = include_str!("../input1.txt");
    println!(
        "{}",
        input
            .lines()
            .map(parse_line2)
            .map(|g| g.power())
            .sum::<u32>()
    );
}

#[derive(Debug, Default, PartialEq)]
struct GameResult {
    id: u32,
    blue: u32,
    red: u32,
    green: u32,

    min_blue: u32,
    min_green: u32,
    min_red: u32,
}

impl GameResult {
    fn power(&self) -> u32 {
        self.min_blue * self.min_red * self.min_green
    }
}

fn parse_line2(line: &str) -> GameResult {
    let (_game, tail) = parse_tag(line, "Game ");
    let (num, tail) = parse_num(tail);
    let id = num.parse::<u32>();
    let (_colon, tail) = parse_tag(tail, ": ");
    let games = tail.split(';');
    let mut game_result = GameResult {
        id: id.unwrap(),
        ..Default::default()
    };
    for game in games {
        let res = game.split(',');
        for entry in res {
            let entry = entry.trim();
            let (num, tail) = parse_num(entry);
            let (_space, tail) = parse_tag(tail, " ");
            if let ("red", _) = dbg!(parse_tag(tail, "red")) {
                let n: u32 = dbg!(num.parse().unwrap());
                if n > game_result.min_red {
                    game_result.min_red = n;
                }
                game_result.red += n;
                continue;
            }
            if let ("green", _) = dbg!(parse_tag(tail, "green")) {
                let n: u32 = dbg!(num.parse().unwrap());
                if n > game_result.min_green {
                    game_result.min_green = n;
                }
                game_result.green += n;
                continue;
            }
            if let ("blue", _) = dbg!(parse_tag(tail, "blue")) {
                let n: u32 = dbg!(num.parse().unwrap());
                if n > game_result.min_blue {
                    game_result.min_blue = n;
                }
                game_result.blue += n;
                continue;
            }
        }
        println!("================================");
    }
    game_result
}

fn parse_line(line: &str) -> Result<GameResult> {
    let (_game, tail) = parse_tag(line, "Game ");
    let (num, tail) = parse_num(tail);
    let id = num.parse::<u32>();
    let (_colon, tail) = parse_tag(tail, ": ");
    let games = tail.split(';');
    let mut game_result = GameResult {
        id: id.unwrap(),
        ..Default::default()
    };
    for game in games {
        let res = game.split(',');
        for entry in res {
            let entry = entry.trim();
            let (num, tail) = parse_num(entry);
            let (_space, tail) = parse_tag(tail, " ");
            if let ("red", _) = dbg!(parse_tag(tail, "red")) {
                let n: u32 = dbg!(num.parse().unwrap());
                if n > 12 {
                    bail!("too many reds {}", n);
                }
                game_result.red += n;
                continue;
            }
            if let ("green", _) = dbg!(parse_tag(tail, "green")) {
                let n: u32 = dbg!(num.parse().unwrap());
                if n > 13 {
                    bail!("too many greens {}", n);
                }
                game_result.green += n;
                continue;
            }
            if let ("blue", _) = dbg!(parse_tag(tail, "blue")) {
                let n: u32 = dbg!(num.parse().unwrap());
                if n > 14 {
                    bail!("too many blues {}", n);
                }
                game_result.blue += n;
                continue;
            }
        }
        println!("================================");
    }
    Ok(dbg!(game_result))
}

fn parse_tag<'input>(input: &'input str, tag: &'input str) -> (&'input str, &'input str) {
    if tag.len() > input.len() {
        return ("", input);
    }
    if &input[0..tag.len()] == tag {
        (tag, &input[tag.len()..])
    } else {
        ("", input)
    }
}

fn parse_num(input: &str) -> (&str, &str) {
    let nums = input.chars().take_while(|c| c.is_ascii_digit()).count();
    (&input[0..nums], &input[nums..])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_tag() {
        let line =
            "Game 1: 2 green, 6 blue, 7 red; 12 green, 6 blue, 3 red; 5 red, 18 green, 4 blue";
        let (game_token, tail) = parse_tag(line, "Game");
        assert_eq!(game_token, "Game");

        assert_eq!(
            " 1: 2 green, 6 blue, 7 red; 12 green, 6 blue, 3 red; 5 red, 18 green, 4 blue",
            tail
        );
    }

    #[test]
    fn it_parses_line() {
        let g = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        assert_eq!(
            parse_line(g).unwrap(),
            GameResult {
                id: 1,
                blue: 9,
                red: 5,
                green: 4,
                ..Default::default()
            }
        );
        let g = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
        assert_eq!(
            parse_line(g).unwrap(),
            GameResult {
                id: 2,
                blue: 6,
                red: 1,
                green: 6,
                ..Default::default()
            }
        );
        let g = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";

        assert_eq!(parse_line(g).unwrap_err().to_string(), "too many reds 20",);
        let g = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red";
        assert_eq!(parse_line(g).unwrap_err().to_string(), "too many blues 15",);
        let g = "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(
            parse_line(g).unwrap(),
            GameResult {
                id: 5,
                blue: 1 + 2,
                red: 6 + 1,
                green: 3 + 2,
                ..Default::default()
            }
        );
    }
}
