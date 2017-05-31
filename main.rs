#[derive(Debug)]
struct LispVal {
    val: Box<SExp>
}

#[derive(Debug)]
enum SExp {
    Nil,
    Cons(Box<SExp>, Box<SExp>),
    Int(i64),
    Float(f64),
    Sym(String),
    Str(String)
}

impl LispVal {
    fn nil() -> Self {
        LispVal { val: Box::new(SExp::Nil) }
    }

    fn cons(car: LispVal, cdr: LispVal) -> Self {
        LispVal { val: Box::new(SExp::Cons(car.val, cdr.val)) }
    }

    fn int(x: i64) -> Self {
        LispVal { val: Box::new(SExp::Int(x)) }
    }

    fn float(x: f64) -> Self {
        LispVal { val: Box::new(SExp::Float(x)) }
    }

    fn sym(s: &str) -> Self {
        LispVal { val: Box::new(SExp::Sym(s.to_string())) }
    }

    fn str(s: &str) -> Self {
        LispVal { val: Box::new(SExp::Str(s.to_string())) }
    }
}

impl std::fmt::Display for LispVal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.val.fmt(f)
    }
}

impl std::fmt::Display for SExp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            SExp::Nil => write!(f, "()"),
            SExp::Cons(ref car, ref cdr) => {
                unimplemented!()
            }
            SExp::Int(ref x) => write!(f, "{}", x),
            SExp::Float(ref x) => write!(f, "{}", x),
            SExp::Sym(ref s) => write!(f, "{}", s),
            SExp::Str(ref s) => write!(f, "{:?}", s)
        }
    }
}

fn main() {
    let d = LispVal::str("Hello\n\"world!\"");
    println!("{}", d);
}
