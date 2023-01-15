use std::collections::HashMap;

fn main() {
    
}

pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }
    let mut counts = [0; 26];

    for (c1, c2) in s.chars().zip(t.chars()) {
        counts[(c1 as u8 - b'a') as usize] += 1;
        counts[(c2 as u8 - b'a') as usize] -= 1;
    }

    return counts.iter().all(|&x| x == 0);
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut complements: HashMap<i32, i32> = HashMap::new();
    for i in 0..nums.len() {
        match complements.get(&nums[i]) {
            Some(&x) => return vec![x, i as i32],
            None => complements.insert(target - nums[i], i as i32),
        };
    }
    panic!("should have atleast one answer");
}

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    strs.into_iter()
    .fold(std::collections::HashMap::<[i32; 26], Vec<String>>::new(), |mut groups, curr| {
        let key = curr.bytes()
            .map(|c| (c - b'a') as usize)
            .fold([0; 26], |mut freqs, c| {
                freqs[c] += 1;
                freqs
            });
        groups.entry(key).or_default().push(curr);
        groups
    }).into_values().collect()
}

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let frequencies = nums.iter()
        .fold(HashMap::<&i32, i32>::new(), |mut acc, x| {
            match acc.get(x) {
                Some(&y) => acc.insert(x, y),
                None => acc.insert(x, 1),
            };
            acc
        });
    let mut nums_copy = nums.to_owned();
    nums_copy.sort_by_key(|x| frequencies.get(x).unwrap());
    nums_copy.dedup();
    nums_copy.into_iter().take(k as usize).collect()
}