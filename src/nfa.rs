//
// nfa.rs
// Nondeterministic Finite Automata
//

use std::fmt;
use std::fs::File;
use std::io::Write;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::collections::BTreeSet;

pub type StateID = usize;

pub struct State {
    //id: StateID,
    accept: bool,
    trans: Vec<Transition>
}

impl State {
    // finds the outgoing transitions for a given label
    fn find_transition(&self, label: Label) -> Vec<&Transition> {
        let mut res : Vec<&Transition> = Vec::new();
        for t in self.trans.iter() {
            if t.label == label {
                res.push(&t);
            }
        }

        res
    }

    fn step(&self, c: char) -> BTreeSet<StateID> {
        let mut res : BTreeSet<StateID> = BTreeSet::new();
        for t in self.trans.iter() {
            match t.label {
                Label::Any => res.insert(t.target),
                Label::Symbol(s) if s == c => res.insert(t.target),
                _ => true
            };
        }

        res
    }
}

#[derive(PartialEq, Eq, Debug, Hash, PartialOrd, Ord, Clone)]
pub enum Label {
    Epsilon,
    Any,
    Symbol(char)
}

impl Label {
    fn to_string(&self) -> String {
        let mut res = String::new();
        match *self {
            Label::Any => res.push('*'),
            Label::Epsilon => res.push_str("&epsilon;"),
            Label::Symbol(c) => res.push(c)
        }
        res
    }
}

impl fmt::Display for Label {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

struct Transition {
    label: Label,
    target: StateID
}


#[test]
fn states() {
    let mut ns = NFABuilder::new();
    let id1 = ns.new_state();
    let id2 = ns.new_state();
    assert_eq!(id1, 0);
    assert_eq!(id2, 1);

    assert!(ns.check_id(id1));
    assert!(ns.check_id(id2));
    assert!(!ns.check_id(100));
}

#[test]
fn add_transition() {
    let mut ns = NFABuilder::new();
    let id1 = ns.new_state();
    let id2 = ns.new_state();
    let id3 = ns.new_state();
    assert!(ns.add_transition(id1, id2, Label::Epsilon));
    assert!(ns.add_transition(id2, id3, Label::Symbol('a')));

    assert!(!ns.add_transition(id1, 1200, Label::Epsilon));
    assert!(!ns.add_transition(1020, id2, Label::Epsilon));
    assert!(!ns.add_transition(1320, 1200, Label::Epsilon));
}

#[test]
fn transitions() {
    let mut ns = NFABuilder::new();
    let id1 = ns.new_state();
    let id2 = ns.new_state();
    let id3 = ns.new_state();
    assert!(ns.add_transition(id1, id2, Label::Epsilon));
    assert!(ns.add_transition(id1, id3, Label::Epsilon));
    assert!(ns.add_transition(id2, id3, Label::Symbol('a')));


    let st1 = ns.get_state(id1).unwrap();
    let t0 = st1.find_transition(Label::Symbol('b'));
    assert_eq!(t0.len(), 0);
    let t1 = st1.find_transition(Label::Epsilon);
    assert_eq!(t1.len(), 2);

    let st2 = ns.get_state(t1[0].target).unwrap();
    let t2 = st2.find_transition(Label::Symbol('a'));
    assert_eq!(t2.len(), 1);

    assert_eq!(t2[0].target, id3);
    assert_eq!(t1[1].target, id3);
}

fn canonical_set_string(bset: &BTreeSet<StateID>) -> String {
    let mut res = String::new();

    for id in bset.iter() {
        res.push_str(&format!("{} ", id));
    }

    res
}


pub struct NFA {
    start: StateID,
    //accept: StateID,
    states: Vec<State>,
    //tables: Vec<HashMap<Label, Vec<StateID>>>
}

impl NFA {
    pub fn get_state(&self, sid: StateID) -> Option<&State> {
        self.states.get(sid)
    }

    fn epsilon_closure(&self, states: &BTreeSet<StateID>) -> BTreeSet<StateID> {
        let mut clos : BTreeSet<StateID> = BTreeSet::new();
        let mut stack : Vec<StateID> = Vec::with_capacity(states.len());

        stack.extend(states.clone());
        clos.extend(states.clone());

        while !stack.is_empty() {
            let s = stack.pop().unwrap();         // safe to unwrap because stack is not empty
            let st = self.get_state(s).unwrap();  // safe to unwrap because states on the stack must exist
            for t in st.find_transition(Label::Epsilon).iter() {
                if !clos.contains(&t.target) {
                    clos.insert(t.target);
                    stack.push(t.target);
                }
            }
        }

        clos
    }

    fn step(&self, s: StateID, c: char) -> BTreeSet<StateID> {
        let st = self.get_state(s).unwrap();
        st.step(c)
    }

    fn steps(&self, vs: &BTreeSet<StateID>, c: char) -> BTreeSet<StateID> {
        let mut res : BTreeSet<StateID> = BTreeSet::new();
        for s in vs.iter() {
            let ss = self.step(*s, c);
            res = res.union(&ss).cloned().collect();
        }

        res
    }

    // this is terribly inefficient for the moment, will
    // optimize if/when necessary
    pub fn simulate(&self, word: &str) -> bool {
        let mut s = self.epsilon_closure(&state_set(self.start));
        for c in word.chars() {
            s = self.epsilon_closure(&self.steps(&s, c));
        }
        s.iter().any(|&i| self.get_state(i).unwrap().accept)
        //s.contains(&self.accept)
    }

    fn transitions_from_set(&self, set: &BTreeSet<StateID>) -> BTreeSet<&Label> {
        let mut labels = BTreeSet::new();
        for sid in set.iter() {
            let state = self.get_state(*sid).unwrap();
            for trans in state.trans.iter() {
                labels.insert(&trans.label);
            }
        }
        labels
    }

    // fn set_state(set: &BTreeSet<StateID>, builder: &mut NFABuilder, map: &mut HashMap<String, StateID>) -> StateID {
    //     match map.contains(canonical_set_string(set)) {
    //         None => {
    //             let state = builder.new_state();
    //             map.insert(set, state);
    //             state
    //         },
    //         Some(state) => state
    //     }
    // }

    pub fn to_dfa(&self) -> Self {
        let mut builder = NFABuilder::new();
        let mut queue = VecDeque::new();
        //let mut marked = Vec::new();
        let mut state_map = HashMap::new();

        let startset = self.epsilon_closure(&state_set(self.start));
        let start = builder.new_state();
        state_map.insert(canonical_set_string(&startset), start);
        queue.push_back(startset);

        while let Some(set) = queue.pop_front() {
            //println!("set in queue: {}", canonical_set_string(&set));
            let state = *(state_map.get(&canonical_set_string(&set)).unwrap());
            for label in self.transitions_from_set(&set) {
                // for each transition from set
                // check if resulting set is already in map; if not, create new state and put in map
                match *label {
                    Label::Epsilon => (),    // do nothing for epsilon transitions, will be picked up in epsilon_closure
                    Label::Symbol(c) => {
                        let new_set = self.epsilon_closure(&self.steps(&set, c));
                        let new_state_opt = state_map.get(&canonical_set_string(&new_set)).map(|state| *state);
                        let new_state = match new_state_opt {
                            None => {
                                let state = builder.new_state();
                                if new_set.iter().any(|&i| self.get_state(i).unwrap().accept) {
                                    builder.set_accepting(state);
                                }
                                state_map.insert(canonical_set_string(&new_set), state);
                                queue.push_back(new_set);
                                state
                            },
                            Some(state) => state
                        };

                        builder.add_transition(state, new_state, Label::Symbol(c));
                    },
                    Label::Any => ()  // TODO
                }
            }
        }

        NFA { states: builder.states, start: start }
    }

    pub fn dot_output(&self, filename: &str) {
        let mut buffer = File::create(filename).unwrap();

        buffer.write(b"digraph {\n").unwrap();
        buffer.write(b"  graph [rankdir=LR]\n").unwrap();
        buffer.write(b"  node [shape=circle]\n").unwrap();
        // loop over state ids
        for sid in 0 .. self.states.len() {
            for trans in &self.states[sid].trans {
                if self.get_state(trans.target).unwrap().accept {
                    buffer.write(format!("  {} [shape=doublecircle]\n", trans.target).as_bytes()).unwrap();
                }
                buffer.write(format!("  {} -> {} [label = \"{}\"]\n", sid,
                                     trans.target, trans.label).as_bytes()).unwrap();
            }
        }
        buffer.write(b"  p [shape=point, style=invis]\n").unwrap();
        buffer.write(format!("  p -> {}\n", self.start).as_bytes()).unwrap();
        buffer.write(b"}\n").unwrap();
    }
}

#[test]
fn canonical_string() {
    let mut set : BTreeSet<StateID> = BTreeSet::new();

    set.insert(15);
    set.insert(12);
    set.insert(8);

    assert!(set.contains(&12));
    assert_eq!(canonical_set_string(&set), "8 12 15 ");
}

fn state_set(s: StateID) -> BTreeSet<StateID> {
    let mut hs : BTreeSet<StateID> = BTreeSet::new();
    hs.insert(s);
    hs
}

// specification for a NFA built by translation from a regexp
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Spec {
    Single(Label),
    Union(Box<Spec>, Box<Spec>),
    Concat(Box<Spec>, Box<Spec>),
    Star(Box<Spec>)
}

impl Spec {
    // helper methods for building specs
    pub fn single(l: Label) -> Box<Spec> {
        Box::new(Spec::Single(l))
    }

    pub fn union(s1: Box<Spec>, s2: Box<Spec>) -> Box<Spec> {
        Box::new(Spec::Union(s1, s2))
    }

    pub fn concat(s1: Box<Spec>, s2: Box<Spec>) -> Box<Spec> {
        Box::new(Spec::Concat(s1, s2))
    }

    pub fn star(s: Box<Spec>) -> Box<Spec> {
        Box::new(Spec::Star(s))
    }
}

struct NFAid {
    start: StateID,
    accept: StateID
}

pub struct NFABuilder {
    states: Vec<State>
}

impl NFABuilder {
    fn new() -> NFABuilder {
        let st : Vec<State> = Vec::with_capacity(10);
        NFABuilder { states: st }
    }

    pub fn build_from_spec(s: Spec) -> NFA {
        let st : Vec<State> = Vec::with_capacity(10);
        let mut builder = NFABuilder { states: st };

        let nid = builder.build(s);
        builder.set_accepting(nid.accept);

        NFA { states: builder.states, start: nid.start }
    }

    // build NFA integrating all the specs; consumes (drains) the vector ss
    pub fn build_from_specs(ss: &mut Vec<Spec>) -> NFA {
        let mut builder = NFABuilder::new();
        let ns = ss.drain(..).map(|s| builder.build(s)).collect();
        let start = builder.fuse_nfas(ns);

        NFA { states: builder.states, start: start }
    }

    fn build(&mut self, s: Spec) -> NFAid {
        match s {
            Spec::Single(l) => self.single(l),
            Spec::Union(s1, s2) => {
                let n1 = self.build(*s1);
                let n2 = self.build(*s2);
                self.union(n1, n2)
            },
            Spec::Concat(s1, s2) => {
                let n1 = self.build(*s1);
                let n2 = self.build(*s2);
                self.concat(n1, n2)
            },
            Spec::Star(s) => {
                let n = self.build(*s);
                self.star(n)
            }
        }
    }

    // fuse all nfas creating a new start state; return ID of new start state
    fn fuse_nfas(&mut self, ns: Vec<NFAid>) -> StateID {
        let nstart = self.new_state();
        for n in ns {
            self.add_transition(nstart, n.start, Label::Epsilon);
            self.set_accepting(n.accept);
        }

        nstart
    }

    fn new_state(&mut self) -> StateID {
        let res = self.states.len();
        let trs : Vec<Transition> = Vec::with_capacity(2);
        //let st = State { id: res, trans: trs, accept: false };
        let st = State { trans: trs, accept: false };

        self.states.push(st);
        res
    }

    fn check_id(&self, id: StateID) -> bool {
        id < self.states.len()
    }

    fn get_state_mut(&mut self, id: StateID) -> &mut State {
        // TODO: change to safe, checked access
        unsafe {
            let res : &mut State = self.states.get_unchecked_mut(id);
            res
        }
    }

    // TODO: check for deletion
    fn get_state(&self, id: StateID) -> Option<&State> {
        self.states.get(id)
    }

    fn set_accepting(&mut self, id: StateID) {
        let st = self.get_state_mut(id);
        st.accept = true;
    }

    fn add_transition(&mut self, src_id: StateID, dst_id: StateID, label: Label) -> bool {
        if self.check_id(src_id) && self.check_id(dst_id) {
            let src_st = self.get_state_mut(src_id);
            let trans = Transition { label: label, target: dst_id };
            src_st.trans.push(trans);
            true
        }
        else {
            false
        }
    }

    fn single(&mut self, l: Label) -> NFAid {
        let st = self.new_state();
        let acc = self.new_state();

        self.add_transition(st, acc, l);

        NFAid { start: st, accept: acc }
    }

    fn union(&mut self, n1: NFAid, n2: NFAid) -> NFAid {
        let start = self.new_state();
        self.add_transition(start, n1.start, Label::Epsilon);
        self.add_transition(start, n2.start, Label::Epsilon);

        let acc = self.new_state();
        self.add_transition(n1.accept, acc, Label::Epsilon);
        self.add_transition(n2.accept, acc, Label::Epsilon);

        NFAid { start: start, accept: acc }
    }

    fn concat(&mut self, n1: NFAid, n2: NFAid) -> NFAid {
        self.add_transition(n1.accept, n2.start, Label::Epsilon);
        NFAid { start: n1.start, accept: n2.accept }
    }

    fn star(&mut self, n: NFAid) -> NFAid {
        let start = self.new_state();
        let acc = self.new_state();

        // transitions connecting old and new start/accept states
        self.add_transition(start, n.start, Label::Epsilon);
        self.add_transition(n.accept, acc, Label::Epsilon);

        // transition connecting start and accept for 0 occurrences
        self.add_transition(start, acc, Label::Epsilon);

        // loop transition for any number of occurrences
        self.add_transition(n.accept, n.start, Label::Epsilon);

        NFAid { start: start, accept: acc }
    }

}

#[test]
fn test_single() {
    let n1 = NFABuilder::build_from_spec(Spec::Single(Label::Epsilon));

    let s0 = n1.get_state(n1.start).unwrap();
    assert_eq!(s0.trans.len(), 1);
    assert_eq!(s0.trans[0].label, Label::Epsilon);

    let ts = s0.find_transition(Label::Epsilon);
    assert_eq!(ts.len(), 1);
    assert!(n1.get_state(ts[0].target).unwrap().accept);

    let s1 = n1.get_state(ts[0].target).unwrap();
    assert_eq!(s1.trans.len(), 0);
    let t1 = s1.find_transition(Label::Epsilon);
    assert_eq!(t1.len(), 0);
}

#[test]
fn test_union() {
    use nfa::Label::{Epsilon, Any};

    let spec = Spec::union(Spec::single(Epsilon), Spec::single(Any));
    let nfa = NFABuilder::build_from_spec(*spec);
    nfa.dot_output("union1.dot");

    let s0 = nfa.get_state(nfa.start).unwrap();
    assert_eq!(s0.trans.len(), 2);
    assert_eq!(s0.trans[0].label, Epsilon);
    assert_eq!(s0.trans[1].label, Epsilon);

    let t0 = s0.find_transition(Epsilon);
    assert_eq!(t0.len(), 2);

    let s1 = nfa.get_state(t0[0].target).unwrap();
    assert_eq!(s1.trans.len(), 1);
    let s2 = nfa.get_state(t0[1].target).unwrap();
    assert_eq!(s2.trans.len(), 1);

    let s3 = nfa.get_state(s1.trans[0].target).unwrap();
    assert_eq!(s3.trans.len(), 1);
    assert_eq!(s3.trans[0].label, Epsilon);
    let s4 = nfa.get_state(s2.trans[0].target).unwrap();
    assert_eq!(s4.trans.len(), 1);
    assert_eq!(s4.trans[0].label, Epsilon);

    assert_eq!(s3.trans[0].target, s4.trans[0].target);
    assert!(nfa.get_state(s3.trans[0].target).unwrap().accept);

    let acc = nfa.get_state(s4.trans[0].target).unwrap();
    assert_eq!(acc.trans.len(), 0);
}

#[test]
fn to_dfa() {
    use nfa::Label::{Epsilon, Any};

    let spec = Spec::union(Spec::single(Label::Symbol('a')), Spec::single(Label::Symbol('b')));
    let nfa = NFABuilder::build_from_spec(*spec);
    nfa.dot_output("todfa_nfa.dot");

    let dfa = nfa.to_dfa();
    dfa.dot_output("todfa_dfa.dot");
}

/*
Testing if a nfa and a dfa are equivalent
from card: https://trello.com/c/q4DyS8jj
*/
#[test]
fn test_nfa_dfa() {
    use regex_syntax::Expr;
    use rules;

    //test 1
    let re1 = Expr::parse(r"a{4,}").unwrap();
    let spc1 = rules::regex_to_nfa_spec(&re1);
    let nfa1 = NFABuilder::build_from_spec(spc1);
    let dfa1 = nfa1.to_dfa();
    assert!(nfa1.simulate("aaaaa"));
    assert!(dfa1.simulate("aaaaa"));
    assert!(!nfa1.simulate("aa"));
    assert!(!dfa1.simulate("aa"));

    //test 2
    let re2 = Expr::parse(r"ab|cd").unwrap();
    let spc2 = rules::regex_to_nfa_spec(&re2);
    let nfa2 = NFABuilder::build_from_spec(spc2);
    let dfa2 = nfa2.to_dfa();
    assert!(nfa2.simulate("ab"));
    assert!(dfa2.simulate("ab"));
    assert!(nfa2.simulate("cd"));
    assert!(dfa2.simulate("cd"));
    assert!(!nfa2.simulate("abcd"));
    assert!(!dfa2.simulate("abcd"));

    //test 3
    let re3 = Expr::parse(r"ab*").unwrap();
    let spc3 = rules::regex_to_nfa_spec(&re3);
    println!("regex!!");
    println!("{:?}", spc3);
    let nfa3 = NFABuilder::build_from_spec(spc3);
    let dfa3 = nfa3.to_dfa();
    nfa3.dot_output("test3_nfa.dot");
    dfa3.dot_output("test3_dfa.dot");

    assert!(nfa3.simulate("abbbbb"));
    assert!(dfa3.simulate("abbbbb"));
    assert!(nfa3.simulate("a"));
    assert!(dfa3.simulate("a"));
    assert!(!nfa3.simulate("abbabbbb"));
    assert!(!dfa3.simulate("abbabbbb"));

}

#[test]
fn test_eps_clos() {
    use nfa::Label::{Epsilon,Any};

    let n1 = NFABuilder::build_from_spec(Spec::Single(Any));
    let cls1 = n1.epsilon_closure(&state_set(n1.start));
    assert_eq!(cls1.len(), 1);
    assert!(cls1.contains(&n1.start));

    let n2 = NFABuilder::build_from_spec(Spec::Single(Epsilon));
    let cls2 = n2.epsilon_closure(&state_set(n2.start));
    assert_eq!(cls2.len(), 2);
    assert!(cls2.contains(&n2.start));
    //assert!(cls2.contains(&n2.accept));
}

#[test]
fn test_simulation() {
    use nfa::Label::*;

    let sp1 = Spec::single(Symbol('a'));

    let n1 = NFABuilder::build_from_spec(*sp1);
    assert!(n1.simulate("a"));
    assert!(!n1.simulate("x"));
    assert!(!n1.simulate("aaa"));

    let sym_a = Spec::single(Symbol('a'));
    let sym_b = Spec::single(Symbol('b'));
    let sp2 = Spec::star(Spec::union(sym_a, sym_b));
    let n2 = NFABuilder::build_from_spec(*sp2);
    assert!(n2.simulate("aabb"));
    assert!(n2.simulate("bbaa"));
    assert!(n2.simulate(""));
    assert!(n2.simulate("aaaaaaaba"));
    assert!(!n2.simulate("aaaaaaabbbbcaaa"));
}

#[test]
fn test_build_specs() {
    use nfa::Label::*;

    let sp1 = Spec::single(Symbol('a'));
    let sp2 = Spec::concat(Spec::single(Symbol('b')), Spec::single(Symbol('a')));

    let mut v : Vec<Spec> = Vec::with_capacity(2);
    v.push(*sp1);
    v.push(*sp2);

    let n = NFABuilder::build_from_specs(&mut v);

    assert!(n.simulate("ba"));
    assert!(n.simulate("a"));
    assert!(!n.simulate("baa"));
}
