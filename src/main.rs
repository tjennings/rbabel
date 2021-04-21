#[derive(Debug)]
enum Bencode {
    Int(i32),
    List(Vec<Bencode>)
}

 // impl fmt::Display for Bencode {
 //   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
 //       let printable = match *self {
 //           Bencode::Int(intv) => write!(f, "{}", intv),
 //           Bencode::List(vecv) => write!(f, "{}", vecv.join(',')),
 //           Bencode::None => write!(f, "")
 //       };
 //       return printable;
 //   }
 // }

fn parse_int(chars: &mut Vec<char>) -> Option<Bencode>{
    let e_idx = chars.iter().rposition(|&e| e == 'e').unwrap();
    let int_chars = &chars[e_idx + 1..chars.len()];
    //println!("{:?}", int_chars);
    //TODO - we should use the ret val here to decide if we return none?
    let int_val = int_chars.iter().rev().collect::<String>().parse::<i32>().unwrap();
    chars.drain(e_idx .. chars.len());
    return Some(Bencode::Int(int_val));
}

fn parse(input: &str) -> Bencode {
    let mut chars: Vec<_> = String::from(input).chars().rev().collect();
    return __parse(&mut chars).unwrap();
}

fn __parse(chars: &mut Vec<char>) -> Option<Bencode> {
    let mut ele = chars.pop();
    let mut ret_val: Option<Bencode> = None;
    if ele == Some('i') {
        ret_val = parse_int(chars);
    } else if ele == Some('l') {
      let mut accum:Vec<Bencode> = Vec::new();
        ele = chars.pop();
        while ele != None && ele != Some('e') {
            chars.push(ele.unwrap());
            if let Some(val) = __parse(chars) {
                accum.push(val);
            }
            ele = chars.pop();
        }
        return Some(Bencode::List(accum));
    }
    return ret_val;
}

fn main() {
    println!("{:?}", parse("i10e"));
    println!("{:?}", parse("le"));
    println!("{:?}", parse("li10ee"));
    println!("{:?}", parse("li10ei15ee"));
    println!("{:?}", parse("lleli8eee"));
}
