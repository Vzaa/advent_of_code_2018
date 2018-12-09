use std::fs;

fn part1(vals: &[usize]) -> (usize, usize) {
    let (children, meta_cnt) = (vals[0], vals[1]);
    let mut tot_len = 2;
    let mut child_sum = 0;

    for _ in 0..children {
        let (l, s) = part1(&vals[tot_len..]);
        tot_len += l;
        child_sum += s;
    }

    let own_sum: usize = vals[tot_len..].iter().take(meta_cnt).sum();

    (tot_len + meta_cnt, child_sum + own_sum)
}

fn part2(vals: &[usize]) -> (usize, usize) {
    let (children, meta_cnt) = (vals[0], vals[1]);
    let mut tot_len = 2;

    let mut child_ids = vec![];
    for _ in 0..children {
        let (l, id) = part2(&vals[tot_len..]);
        tot_len += l;
        child_ids.push(id);
    }

    let metadata = vals[tot_len..].iter().take(meta_cnt);

    if children == 0 {
        let own_sum: usize = metadata.sum();
        (tot_len + meta_cnt, own_sum)
    } else {
        let ch_sum: usize = metadata.map(|m| child_ids.get(*m - 1).unwrap_or(&0)).sum();
        (tot_len + meta_cnt, ch_sum)
    }
}

fn main() {
    let dat = fs::read_to_string("input").unwrap();
    let vals: Vec<usize> = dat.split_whitespace().map(|x| x.parse().unwrap()).collect();

    let fin = part1(&vals);
    println!("Part 1: {}", fin.1);

    let fin = part2(&vals);
    println!("Part 2: {}", fin.1);
}
