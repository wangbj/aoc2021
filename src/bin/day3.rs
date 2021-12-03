use std::io::{self, BufRead};

#[derive(Clone, Debug)]
struct Input {
    bits: Vec<u64>,
    bit_size: usize,
}

impl Input {
    fn from_stdin() -> Self {
        let stdin = io::stdin();
        let bits: Vec<(usize, u64)> = stdin
            .lock()
            .lines()
            .filter_map(|l| {
		let line = l.ok()?;
                Some((line.len(), u64::from_str_radix(&line, 2).ok()?))
            })
            .collect();
	let bit_size = bits[0].0;
	let bits = bits.iter().map(|x| x.1).collect();
	Input {
	    bits,
	    bit_size,
	}
    }
}

fn part1(input: &Input) -> i64 {
    let bit_size = input.bit_size;
    let mut res = vec![0; bit_size];
    let nb = input.bits.len();
    input.bits.iter().for_each(|x| {
	for i in 0..bit_size {
	    if x & (1 << i) == 1 << i {
		res[i] += 1;
	    } else {
	    }
	}
    });
    let (gamma, epsilon) = res.iter().enumerate().fold((0, 0), |acc, (k, &x)| {
	if x > nb / 2 {
	    (acc.0 | (1 << k), acc.1)
	} else {
	    (acc.0, acc.1 | (1 << k))
	}
    });
    gamma * epsilon
}

#[derive(Clone, Copy, Debug)]
enum CheckPolicy {
    OneMajority,
    ZeroMajority,
    JustMajority,
    JustMinority,
}

#[derive(Clone, Debug)]
enum Res {
    More(Vec<u64>),
    Done(u64),
}

fn next_round(bits: &[u64], k: usize, pol: CheckPolicy) -> (CheckPolicy, Res) {
    let mut one_count: usize = 0;
    let nb = bits.len();
    bits.iter().for_each(|x| {
	if x & (1 << k) != 0 {
	    one_count += 1;
	}
    });
    let zero_count = nb - one_count;
    let major_is_one = one_count >= zero_count;
    let major_is_zero = zero_count >= one_count;
    let next_pol = match pol {
	CheckPolicy::OneMajority => CheckPolicy::JustMajority,
	CheckPolicy::ZeroMajority => CheckPolicy::JustMinority,
	CheckPolicy::JustMajority => CheckPolicy::JustMajority,
	CheckPolicy::JustMinority => CheckPolicy::JustMinority,
	
    };
    let res: Vec<u64> = bits.iter().filter(|&x| {
	match pol {
	    CheckPolicy::OneMajority => major_is_one && (x & 1 << k != 0),
	    CheckPolicy::ZeroMajority => major_is_one && (x & 1 << k == 0),
	    CheckPolicy::JustMajority if major_is_one => (x & 1 << k != 0),
	    CheckPolicy::JustMajority if major_is_zero => (x & 1 << k == 0),
	    CheckPolicy::JustMinority if major_is_one => (x & 1 << k == 0),
	    CheckPolicy::JustMinority if major_is_zero => (x & 1 << k != 0),
	    _ => unreachable!(),
	}
    }).cloned().collect();
    (next_pol,  if res.len() > 1 {
	Res::More(res)
    } else if res.len() == 1 {
	Res::Done(res[0])
    } else {
	unreachable!()
    })
}

fn part2(input: &Input) -> u64 {
    let bit_size = input.bit_size;

    let mut next_vec = input.bits.clone();
    let mut o2 = 0;
    let mut co2 = 0;

    let mut policy = CheckPolicy::OneMajority;
    for k in (0..bit_size).rev().cycle() {
	match next_round(&next_vec, k, policy) {
	    (pol, Res::More(x)) => {
		policy = pol;
		next_vec = x;
	    }
	    (_, Res::Done(x)) => {
		o2 = x;
		break;
	    }
	}
    };

    let mut next_vec = input.bits.clone();
    let mut policy = CheckPolicy::ZeroMajority;
    for k in (0..bit_size).rev().cycle() {
	match next_round(&next_vec, k, policy) {
	    (pol, Res::More(x)) => {
		policy = pol;
		next_vec = x;
	    }
	    (_, Res::Done(x)) => {
		co2 = x;
		break;
	    }
	}
    };
    o2 * co2
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_input() -> Input {
    let bits = r#"
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
"#.lines().filter_map(|l| u64::from_str_radix(l, 2).ok()).collect();
	Input {
	    bits,
	    bit_size: 5,
	}
    }

    #[test]
    fn sample_works() {
	let input = test_input();
	assert_eq!(input.bits.len(), 12);
	assert_eq!(part1(&input), 198);
	assert_eq!(part2(&input), 230);
    }
    
}

fn main() {
    let input = Input::from_stdin();
    println!("{:?}", part1(&input));
    println!("{:?}", part2(&input));
}
