//
// nfa.rs
// Nondeterministic Finite Automata
//

type StateID = usize;

struct State {
    id: StateID,
    accept: bool,
    trans: Vec<Transition>
}

impl State {
    pub fn find_transition(&self, label: Label) -> Option<&Transition> {
        for t in self.trans.iter() {
            if t.label == label {
                return Some(&t);
            }
        }

        None
    }
}

#[derive(PartialEq, Eq)]
enum Label {
    Epsilon,
    Symbol(char)
}

struct Transition {
    label: Label,
    target: StateID
}

struct NFASet {
    states: Vec<State>
}

impl NFASet {
    pub fn new() -> NFASet {
        let st : Vec<State> = Vec::with_capacity(10);
        NFASet { states: st }
    }

    pub fn create_state(&mut self, acc: bool) -> StateID {
        let id = self.states.len();
        let trs : Vec<Transition> = Vec::with_capacity(2);
        let st = State { id: id, accept: acc, trans: trs };
        self.states.push(st);
        id
    }
    
    pub fn check_id(&self, id: StateID) -> bool {
        id < self.states.len()
    }

    fn get_state_mut(&mut self, id: StateID) -> &mut State {
        // TODO: change to safe, checked access
        unsafe {
            let res : &mut State = self.states.get_unchecked_mut(id);
            res
        }
    }

    pub fn get_state(&self, id: StateID) -> Option<&State> {
        self.states.get(id)
    }

    pub fn add_transition(&mut self, src_id: StateID, dst_id: StateID, label: Label) -> bool {
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

    pub fn create_nfa(&self, start: StateID) -> Option<NFA> {
        if self.check_id(start) {
            Some(NFA { start: start })
        }
        else {
            None
        }
    }

    pub fn nfa_union(&mut self, n1: NFA, n2: NFA) -> NFA {
        NFA { start: 0 }  // TODO: implement
    }

    pub fn nfa_concat(&mut self, n1: NFA, n2: NFA) -> NFA {
        NFA { start: 0 }  // TODO: implement
    }

    pub fn nfa_star(&mut self, n: NFA) -> NFA {
        NFA { start: 0 }  // TODO: implement
    }
}

struct NFA {
    start: StateID,
}

#[test]
fn states() {
    let mut ns = NFASet::new();
    let id1 = ns.create_state(false);
    let id2 = ns.create_state(false);
    assert_eq!(id1, 0);
    assert_eq!(id2, 1);

    assert!(ns.check_id(id1));
    assert!(ns.check_id(id2));
    assert!(!ns.check_id(100));
}

#[test]
fn add_transition() {
    let mut ns = NFASet::new();
    let id1 = ns.create_state(false);
    let id2 = ns.create_state(false);
    let id3 = ns.create_state(false);
    assert!(ns.add_transition(id1, id2, Label::Epsilon));
    assert!(ns.add_transition(id2, id3, Label::Symbol('a')));

    assert!(!ns.add_transition(id1, 1200, Label::Epsilon));
    assert!(!ns.add_transition(1020, id2, Label::Epsilon));
    assert!(!ns.add_transition(1320, 1200, Label::Epsilon));
}

#[test]
fn transitions() {
    let mut ns = NFASet::new();
    let id1 = ns.create_state(false);
    let id2 = ns.create_state(false);
    let id3 = ns.create_state(false);
    assert!(ns.add_transition(id1, id2, Label::Epsilon));
    assert!(ns.add_transition(id2, id3, Label::Symbol('a')));
    
    let st1 = ns.get_state(id1).unwrap();
    let t1 = st1.find_transition(Label::Epsilon).unwrap();

    let st2 = ns.get_state(t1.target).unwrap();
    let t2 = st2.find_transition(Label::Symbol('a')).unwrap();

    assert_eq!(t2.target, id3);
}

