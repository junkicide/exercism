pub fn factors(mut m: u64) -> Vec<u64> {
    let mut factors = vec![];
    let primes = primes(m);
    for i in primes {
        while m%i == 0 {
            factors.push(i);
            m = m/i;
        }
        if m==1 { break}
    }
    factors
}



fn primes(n: u64) -> Vec<u64> {
    if n<2 {return vec![]}
    let mut count= 0;
    let lim = (n as f64).sqrt() as u64;
    let mut lst: Vec<u64> = (2..n).collect();
    let mut m = 2;
    while m <= lim {
        count+=1;
        let mut check = lst.split_off(count);
        check = check.into_iter().filter(|&x| x%m != 0).collect::<Vec<_>>();
        lst =[lst, check].concat();
        m = lst[count];
        
    }
    lst
}
