
fn main(){
    let mut max_sum = 0;
    for group in include_str!("data.txt")
        .split("\n\n"){
        let mut current_sum = 0;
        for line in group.lines(){
            let temp = line.parse::<u64>().expect("something went wrong while parsing");
            current_sum += temp;
        }
        if current_sum > max_sum {
            max_sum = current_sum;
        }
    }
    println!("1st problem: {}",max_sum);
    
    let mut sums = Vec::new();
    for group in include_str!("data.txt")
        .split("\n\n"){
        sums.push(group
            .lines()
            .map(|v| v.parse::<u64>().expect("something went wrong while parsing"))
            .sum::<u64>());
        }
//    println!("{}",
//    sums.sort_by_key(|n| std::u64::MAX - n).to_vec()[0..3]
//        .to_vec()
//        .iter()
//        .sum::<u64>());
    sums.sort_by_key(|n| std::u64::MAX - n);
    let mut sum3 = Vec::new();
    sum3 = sums[0..3].to_vec();
    println!("2nd problem: {}",sum3.iter().sum::<u64>());//why does is work and previous one doesnt
                                            //??????????????????
}
