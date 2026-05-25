use std::io;

fn main() {
    println!("Enter the string");

    let mut s = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("Failed to read");

    let fs : &str = first_word(&mut s);
    println!("first word is: {fs}");

    let ls : &str = last_word(&mut s);
    println!("last word is: {ls}");

    let init : String = initials(&mut s);
    println!("initials are: {init}");

}

fn first_word(s : &mut String) -> &str{
    let bytes = s.as_bytes();
    for (i,&item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[..i];
        }
    }
    &s[..]
}

fn last_word(s : &mut String) -> &str{
    let bytes = s.as_bytes();
    for (i,&item) in bytes.iter().enumerate().rev(){
        if item == b' '{
            return &s[i..];
        }
    }
    &s[..]
}

fn initials(s : &mut String) -> String{
    let bytes = s.as_bytes();
    let mut init = String::new();
    init.push_str(&s[..1]);
    for (i,&item) in bytes.iter().enumerate(){
        
        if item == b' '{
            init.push_str(&s[i..i+2]);
        }
    }
    init

}