use regex_syntax::Expr;
use regex_syntax::Repeater;

use nfa::NFA;
use nfa::Spec;
use nfa::NFABuilder;
use nfa::Label;

struct Rule {
    regex: Expr,
    action: String,
}

/// Converts a sequence of characters into a concatenation of simple
/// transitions.
///
/// The return value is a Spec containing the concatenation of all
/// single transition automata generated for each character in `chars`.
fn literal_to_spec(chars: &Vec<char>) -> Spec {
    // take the first
    let mut tmp: Box<Spec> = Spec::single(Label::Symbol((*chars)[0]));

    // iterate for the others
    for i in chars.iter().skip(1) {
        let aux = Spec::single(Label::Symbol(*i));
        tmp = Spec::concat(tmp,aux);
    }

    // return the concatenation
    *tmp
}

/// Converts a concatenation of `Expr` into a concatenation of `Spec`s.
///
/// The return value is a `Spec` containing the concatenation of all
/// REs in `exprs`.
fn concat_to_spec(exprs: &Vec<Expr>) -> Spec {
    let mut tmp: Box<Spec> = Box::new(regex_to_nfa_spec(&(exprs[0])));

    for i in exprs.iter().skip(1) {
        let aux : Box<Spec> = Box::new(regex_to_nfa_spec(&i));
        tmp  = Spec::concat(tmp,aux);
    }

    *tmp
}

/// Converts a union of `Expr` into a union of `Spec`s.
///
/// The return value is a `Spec` containing the union of all
/// REs in `exprs`.
fn alternate_to_spec(exprs: &Vec<Expr>) -> Spec {
    let mut tmp: Box<Spec> = Box::new(regex_to_nfa_spec(&(exprs[0])));;

    for i in exprs.iter().skip(1) {
        let aux : Box<Spec> = Box::new(regex_to_nfa_spec(&i));
        tmp  = Spec::union(tmp,aux);
    }

    *tmp
}

fn repeater_range(s1: Box<Spec>, min: u32, max: Option<u32>) -> Spec {
    let mut begin:u32 = 0;
    let mut aux:Box<Spec>;
    let mut spec:Box<Spec>;

    if min == 0 {
        aux = Spec::single(Label::Epsilon);
    } else {
        aux = s1.clone();
        begin = 1;
    }

    for _ in begin..min {
        aux = Spec::concat(aux, s1.clone());
    }

    if max == None {
        spec = Spec::concat(aux, Spec::star(s1))
    } else {
        spec = aux.clone();

        //iterate min + 1 ... max
        for _ in min..max.unwrap() {
            aux = Spec::concat(aux, s1.clone());
            spec = Spec::union(spec, aux.clone());
        }
    }
    *spec
}

/// Converts `re` and repeater specification `r` into a NFA `Spec`.
///
/// The repetition behavior is translated to the basic constructs
/// of a NFA `Spec`: union, concatenation and Kleene star of NFAs,
/// using the NFA built for `re` as one of the operands.
fn repeater_to_spec(re: &Expr, r: &Repeater) -> Spec {
    let s1 = Box::new(regex_to_nfa_spec(re));

    match *r {
        Repeater::ZeroOrOne => Spec::Union(Spec::single(Label::Epsilon), s1),
        Repeater::ZeroOrMore => Spec::Star(s1),
        Repeater::OneOrMore => Spec::Concat(s1.clone(), Spec::star(s1)),
        Repeater::Range { min, max } => repeater_range(s1, min, max)
    }
}

/// Converts the regexp `re` into a NFA `Spec`.
///
/// The function traverses the `re` syntax tree recursively, building
/// NFA Specs for each component and combining them according to the
/// operations in the ER.
pub fn regex_to_nfa_spec(re: &Expr) -> Spec {
    match *re {
        Expr::AnyChar => Spec::Single(Label::Any),
        Expr::AnyCharNoNL => Spec::Single(Label::Any),  // FIX: don't match newline
        Expr::Repeat { ref e, r, greedy } => repeater_to_spec(&*e, &r),
        Expr::Literal{ ref chars, casei } => literal_to_spec(chars),
        Expr::Concat(ref exprs) => concat_to_spec(&exprs),
        Expr::Alternate(ref exprs) => alternate_to_spec(&exprs),
        _ => unimplemented!()
    }
}

#[test]
fn test_repeater_zero_or_more1() {
    let re = Expr::parse(r".*").unwrap();
    let spec = regex_to_nfa_spec(&re);

    println!("regex: {:?}", re);
    assert_eq!(spec, *Spec::star(Spec::single(Label::Any)));

}

#[test]
fn test_literal() {
    let re = Expr::parse(r"ab").unwrap();
    let spec = regex_to_nfa_spec(&re);

    println!("regex: {:?}", re);
    assert_eq!(spec, *Spec::concat(Spec::single(Label::Symbol('a')), Spec::single(Label::Symbol('b'))));
}

#[test]
fn test_concat(){
    let re1 = Expr::parse(r"ab").unwrap();
    let re2 = Expr::parse(r"cd").unwrap();


    let mut tmp: Vec<Expr> = Vec::with_capacity(2);

    tmp.push(re1);
    tmp.push(re2);

    let aux = concat_to_spec(&tmp);

    let aux2 = NFABuilder::build_from_spec(aux);

    assert!(aux2.simulate("abcd"));
    assert!(!aux2.simulate("ab"));
    assert!(!aux2.simulate("cd"));

}

#[test]
fn test_altenate(){
    let re1 = Expr::parse(r"ab").unwrap();
    let re2 = Expr::parse(r"cd").unwrap();


    let mut tmp: Vec<Expr> = Vec::with_capacity(2);

    tmp.push(re1);
    tmp.push(re2);
    let aux = alternate_to_spec(&tmp);

    let aux2 = NFABuilder::build_from_spec(aux);

    assert!(!aux2.simulate("abcd"));
    assert!(aux2.simulate("ab"));
    assert!(aux2.simulate("cd"));
}

#[test]
fn test_repeater_range() {
    let re = Expr::parse(r"a{0,4}").unwrap();
    let spec = regex_to_nfa_spec(&re);
    let n = NFABuilder::build_from_spec(spec);

    println!("regex: {:?}", re);
    assert!(n.simulate("aaaa"));
    assert!(n.simulate(""));
    assert!(!n.simulate("aaaaaaaaaaaa"));
}
