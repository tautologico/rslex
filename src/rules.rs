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

fn repeater_to_spec(re: &Expr, r: &Repeater) -> Spec {
    let s1 = Box::new(regex_to_nfa_spec(re));

    match *r {
        Repeater::ZeroOrOne => Spec::Union(Spec::single(Label::Epsilon), s1),
        Repeater::ZeroOrMore => Spec::Star(s1),
        Repeater::OneOrMore => Spec::Concat(s1.clone(), Spec::star(s1)),
        Repeater::Range { min, max } => unimplemented!()
    }
}

fn regex_to_nfa_spec(re: &Expr) -> Spec {
    match *re {
        Expr::AnyChar => Spec::Single(Label::Any),
        Expr::AnyCharNoNL => Spec::Single(Label::Any),  // FIX: don't match newline
        Expr::Repeat { ref e, r, greedy } => repeater_to_spec(&*e, &r),
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
