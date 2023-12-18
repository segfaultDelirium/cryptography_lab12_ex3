use itertools::Itertools;


fn main() {

    let p = 13;
    let res = (1..p).into_iter().map(|x| modulo_euclid(x * x, p))
        .unique()
        .sorted()
        .collect::<Vec<_>>();

    println!("reszty kwadratowe w Z13 = {:?}", res);

    let res = (1..p).into_iter().filter(|a| modulo_euclid(a.pow(((p - 1) / 2) as u32), p) == 1)
        // .unique()
        .sorted()
        .collect::<Vec<_>>();
    println!("reszty kwadratowe w Z13 = {:?}", res);


}


fn modulo_euclid(j: i128, k: i128) -> i128 {
    let res =  j % k;
    if res < 0 {return res + k} else {return res}
}
