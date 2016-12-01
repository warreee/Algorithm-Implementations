fn main() {
    let t: i32 = 5;
    println!("Number of doors {:?}", mapper(t,10));
    let init = vec![0;10];
    let r = zips(init, mapper(t,10));
    println!("{:?}",r);
}

fn zips(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    let temp = a.iter().zip(b);
    let mut result = Vec::new();
    for (_,(x,y)) in temp.enumerate() {
        result.push((x + y)%2)
    }
    return result
}


fn mapper(current : i32, n: i32) -> Vec<i32> {
    let doornumbers : Vec<i32> = (1..n+1).collect();
    let mut vec :Vec<i32> = Vec::with_capacity(n as usize);
    doornumbers.iter().map(|&x| vec.push(toggle(x, current))).collect::<Vec<_>>();
    return vec
}

fn toggle(x: i32, i: i32) -> i32{
    if x % i == 0 {
        1
    } else {
        0
    }
}
