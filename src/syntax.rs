struct Ident(String);

enum Term {
    Var(Ident),
    Abs(Ident, Box<Term>),
    App(Box<Term>, Box<Term>),
}
