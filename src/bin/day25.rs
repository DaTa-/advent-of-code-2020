fn main() {
    let input = [11239946, 10464955];

    let pub_keys = input;
    const SUBJECT: u64 = 7;
    const MOD: u64 = 20201227;

    fn bruteforce(pub_key: u64) -> u64 {
        let mut value = 1;
        (1..)
            .find(|_| {
                value = value * SUBJECT % MOD;
                value == pub_key
            })
            .unwrap()
    }

    let priv_keys = [bruteforce(pub_keys[0]), bruteforce(pub_keys[1])];

    fn encrypt(subject: u64, priv_key: u64) -> u64 {
        (0..priv_key).fold(1, |value, _| value * subject % MOD)
    }

    pub_keys
        .iter()
        .zip(&priv_keys)
        .for_each(|(&pub_key, &priv_key)| assert_eq!(pub_key, encrypt(SUBJECT, priv_key)));

    let encryption_keys = [
        encrypt(pub_keys[1], priv_keys[0]),
        encrypt(pub_keys[0], priv_keys[1]),
    ];

    assert_eq!(encryption_keys[0], encryption_keys[1]);
    println!("{}", encryption_keys[0]);
}
