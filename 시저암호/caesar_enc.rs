fn encrypt(text: &str, shift: i16) -> String{
    //'A'와 'Z'의 문자코드를 i16으로 취득
    let code_a = 'A' as i16;
    let code_z = 'Z' as i16;
    //결과를 대입할 변수를 선언

    let mut result = String::new();
    //한글자씩 치환 처리
    for ch in text.chars(){
        let mut code = ch as i16;
        if code_a <= code && code <= code_z{
            //shift만큼 뒤의 문자로 치환
            code = (code - code_a + shift + 26) % 26 + code_a;

        }
        result.push((code as u8) as char); //문자 코드를 다시 문자로 변환
    }
    return result;

}

fn main(){
    let enc = encrypt("I LOVE RUST.", 3);
    let dec = encrypt(&enc, -3);
    println!("{} => {}",enc, dec);
}