//소수인지 확인하는 함수
fn is_prime( n: usize) -> bool {
    for i in 2..n {
        if n % i == 0{
            return false
        }
    }
    return true
}

// 소수 100개를 구하는 함수
fn get_primes(primes: &mut[usize; 100]){
    let mut i = 2;
    let mut count = 0;
    while count < 100{
        if is_prime(i) {
            primes[count] = i;
            count += 1;
        }
        i += 1;
    }
}

fn main() {
    //초깃값이 0인 배열 100개를 준비
    let mut primes = [0; 100];
    //소수 100개를 구함
    get_primes(&mut primes);
    //결과표시
    println!("{:?}", primes);   //배열 타입의 값을 화면에 출력하는 경우 Println! 매크로에 '{}"가 아니라"{:?}"를 사용한다.
}