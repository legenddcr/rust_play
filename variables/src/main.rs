use std::cmp;

fn main() {
    let mut cnt = 0;
    let mut min = 2021;
    let mut max = 0;

    for ou_la in 10..100 {
        for jia_li_lue in 102..1000 {
            for a_ji_mi_de in 1023..2021 {
                if !check_repeatation(ou_la, jia_li_lue, a_ji_mi_de) {
                    continue;
                }

                if ou_la + jia_li_lue + a_ji_mi_de == 2021 {
                    cnt += 1;
                    min = cmp::min(min, a_ji_mi_de);
                    max = cmp::max(max, a_ji_mi_de);
                    // println!("{} - {} - {}", ou_la, jia_li_lue, a_ji_mi_de);

                    // if a_ji_mi_de == 1698 {
                    //     println!("Max: 欧拉 = {}, 伽利略 = {}, 阿基米德 = {}", ou_la, jia_li_lue, a_ji_mi_de)
                    // }
                }
            }
        }
    }

    // Max: 50 - 273 - 1698
    // Max: 53 - 270 - 1698
    // Max: 70 - 253 - 1698
    // Max: 73 - 250 - 1698
    // Total: 264, 最小的阿基米德 = 1026, 最大的阿基米德= 1698

    println!(
        "Total: {}, 最小的阿基米德 = {}, 最大的阿基米德= {}",
        cnt, min, max
    );
}

fn check_repeatation(mut x: i32, mut y: i32, mut z: i32) -> bool {
    assert!(x > 0 && y > 0 && z > 0);

    let mut flag_vecs = vec![false; 10];
    while x != 0 {
        let n = x % 10;
        if flag_vecs[n as usize] {
            return false;
        }
        flag_vecs[n as usize] = true;
        x = x / 10;
    }

    while y != 0 {
        let n = y % 10;
        if flag_vecs[n as usize] {
            return false;
        }
        flag_vecs[n as usize] = true;
        y = y / 10;
    }

    while z != 0 {
        let n = z % 10;
        if flag_vecs[n as usize] {
            return false;
        }
        flag_vecs[n as usize] = true;
        z = z / 10;
    }
    true
    // update_flags(&flag_vecs, x) && update_flags(flag_vecs, y) && update_flags(flag_vecs, z)
}

#[allow(dead_code)]
fn update_flags(flag_vecs: &mut Vec<bool>, mut x: i32) -> bool {
    if x == 0 {
        return true;
    }

    while x != 0 {
        let n = x % 10;
        if flag_vecs[n as usize] {
            return false;
        }
        flag_vecs[n as usize] = true;
        x = x / 10;
    }
    true
}

#[allow(dead_code)]
fn check_each_num(mut x: i32) -> bool {
    if x == 0 {
        return true;
    }

    let mut flag_vecs = vec![false; 10];
    while x != 0 {
        let n = x % 10;
        if flag_vecs[n as usize] {
            return false;
        }
        flag_vecs[n as usize] = true;
        x = x / 10;
    }
    true
}

#[allow(dead_code)]
fn add_each_num(mut x: i32) -> i32 {
    let mut sum = 0;
    while x != 0 {
        sum += x % 10;
        x = x / 10;
    }
    sum
}

#[test]
fn test_add_each_num() {
    assert_eq!(6, add_each_num(15));
    assert_eq!(1, add_each_num(10));
}

#[test]
fn test_check_each_num() {
    assert!(check_each_num(0));
    assert!(check_each_num(1));
    assert!(check_each_num(1234));
    assert!(!check_each_num(1231));
}
