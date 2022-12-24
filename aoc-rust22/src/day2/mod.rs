use std::fs;

fn read_contents() -> String{
    let contents = fs::read_to_string("src/day2/input.txt").expect("Could not read file");

    return contents;
}

fn get_pts_1(opp: char, mov: char) -> u32 {

    let base_score = match mov {
        'X'=>1,
        'Y'=>2,
        'Z'=>3,
        _=>0,
    };

    let win_score = {
        if opp == 'A' {
            match mov {
                'X'=>3,
                'Y'=>6,
                'Z'=>0,
                _=>0,
            }
        } else if opp == 'B' {
            match mov {
                'X'=>0,
                'Y'=>3,
                'Z'=>6,
                _=>0,
            }
        } else {
            match mov {
                'X'=>6,
                'Y'=>0,
                'Z'=>3,
                _=>0,
            }
        }
    };

    return base_score+win_score;
}

fn get_pts_2(opp: char, res: char) -> u32 {

    let mov = {
        if opp == 'A' {
            match res {
                'X'=>'C',
                'Y'=>'A',
                'Z'=>'B',
                _=>'Z',
            }
        } else if opp == 'B' {
            match res {
                'X'=>'A',
                'Y'=>'B',
                'Z'=>'C',
                _=>'Z',
            }
        } else {
            match res {
                'X'=>'B',
                'Y'=>'C',
                'Z'=>'A',
                _=>'Z',
            }
        }
    };

    let base_score = match mov {
        'A'=>1,
        'B'=>2,
        'C'=>3,
        _=>0,
    };

    let win_score = {
        if opp == 'A' {
            match mov {
                'A'=>3,
                'B'=>6,
                'C'=>0,
                _=>0,
            }
        } else if opp == 'B' {
            match mov {
                'A'=>0,
                'B'=>3,
                'C'=>6,
                _=>0,
            }
        } else {
            match mov {
                'A'=>6,
                'B'=>0,
                'C'=>3,
                _=>0,
            }
        }
    };

    return base_score+win_score;
}

pub fn solve() {
    let contents = read_contents();

    let total_1: u32 = contents.split("\n").into_iter().map(|l: &str| {
        let opp = l.chars().nth(0).unwrap();
        let mov = l.chars().nth(2).unwrap();

        get_pts_1(opp, mov)
    }).sum();

    let total_2: u32 = contents.split("\n").into_iter().map(|l: &str| {
        let opp = l.chars().nth(0).unwrap();
        let res = l.chars().nth(2).unwrap();

        get_pts_2(opp, res)
    }).sum();

    println!("Part 1: {}", total_1);
    println!("Part 2: {}", total_2);
}