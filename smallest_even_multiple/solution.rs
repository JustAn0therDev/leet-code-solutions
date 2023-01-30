pub fn smallest_even_multiple(n: i32) -> i32 {

    let mut mutable_n = n;

    while true {

        if (mutable_n % n == 0 && mutable_n % 2 == 0) {
            break;
        }

        mutable_n += 1;
    }

    return mutable_n;
}