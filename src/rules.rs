
use regex_syntax::Expr;
use nfa::NFA;
use nfa::Spec;
use nfa::NFABuilder;
use nfa::Label;

struct Rule {
    regex: Expr,
    action: String,
}

fn translate_rule(r: Rule) -> NFA {
    use nfa::Label::{Epsilon, Any};

    let spec = Spec::union(Spec::single(Epsilon), Spec::single(Any));
    let nfa = NFABuilder::build_from_spec(*spec);
    
    return nfa;
}

#[test]
fn test_re1() {
    let rx = Expr::parse(r"ab|yz").unwrap();
    let rule1 = Rule { regex: rx.clone(), action: "println!(\"ha!\")".to_string() };

    assert_eq!(rule1.regex, rx);
}
