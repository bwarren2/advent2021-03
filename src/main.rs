fn part1() -> isize {
    let data: Vec<i32> = include_str!("input.txt")
        .lines()
        .map(|x| {
            x.chars()
                .map(|digit| digit.to_string().parse::<i32>().unwrap())
                .collect()
        })
        .reduce(|a: Vec<i32>, b: Vec<i32>| -> Vec<i32> {
            a.iter().zip(b.iter()).map(|(&a, &b)| a + b).collect()
        })
        .unwrap();

    let str1 = data
        .iter()
        .map(|x| -> &str {
            if x > &499 {
                "1"
            } else {
                "0"
            }
        })
        .collect::<Vec<&str>>()
        .join("");
    let str2 = data
        .iter()
        .map(|x| -> &str {
            if x > &499 {
                "0"
            } else {
                "1"
            }
        })
        .collect::<Vec<&str>>()
        .join("");

    let int1 = isize::from_str_radix(&str1, 2).unwrap();
    let int2 = isize::from_str_radix(&str2, 2).unwrap();
    return int1 * int2;
}

fn part2<'a>(strings: &'a mut Vec<&str>) -> isize {
    let oxygen = filter_strings(strings, true);
    let co2 = filter_strings(strings, false);

    let int1 = isize::from_str_radix(&oxygen, 2).unwrap();
    let int2 = isize::from_str_radix(&co2, 2).unwrap();

    int1*int2
}

fn filter_strings<'a>(strings: &'a Vec<&str>, prefer_common: bool) -> &'a str {
    let mut position = 0;
    let mut ones: Vec<&str>; 
    let mut subset_strings = strings.to_owned();
    while subset_strings.len() > 1 {
        let zeroes: Vec<&str> = subset_strings
            .iter()
            .filter(|x| x.chars().nth(position).unwrap() == '0')
            .copied()
            .collect::<Vec<&str>>();
        ones = subset_strings
            .iter()
            .filter(|x| x.chars().nth(position).unwrap() == '1')
            .copied()
            .collect::<Vec<&str>>();
        if prefer_common {
            if zeroes.len() == ones.len() {
                subset_strings = ones;
            } else if zeroes.len() >  ones.len() {
                subset_strings = zeroes;
            } else {
                subset_strings = ones;
            }
        } else {
            if zeroes.len() == ones.len() {
                subset_strings = zeroes;
            } else if zeroes.len() >  ones.len() {
                subset_strings = ones;
            } else {
                subset_strings = zeroes;
            }
        }
        position += 1;
    }
    subset_strings[0]
}

fn main() {
    println!("{:?}", part1());

    let mut strings = include_str!("input.txt").lines().collect::<Vec<&str>>();

    let value = part2(&mut strings);
    println!("{:?}", value);
}
