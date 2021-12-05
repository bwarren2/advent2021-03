fn main() {
    let data: Vec<i32> = include_str!("input.txt")
        .lines()
        .map(|x| {
            x.chars()
                .map(|digit| {
                    println!("{:?}", x);
                    digit.to_string().parse::<i32>().unwrap()
                })
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
    println!("{:?}", data);
    println!("{:?}", int1);
    println!("{:?}", int1*int2);
}
