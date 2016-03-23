//
// nfa.rs
// Nondeterministic Finite Automata
//

type StateID = usize;

struct State {
    id: StateID,
    trans: Vec<Transition>
}

impl State {
    // finds the outgoing transitions for a given label
    pub fn find_transition(&self, label: Label) -> Vec<&Transition> {
        let mut res : Vec<&Transition> = Vec::new();
        for t in self.trans.iter() {
            if t.label == label {
                res.push(&t);
            }
        }

        res
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

    pub fn create_state(&mut self) -> StateID {
        let id = self.states.len();
        let trs : Vec<Transition> = Vec::with_capacity(2);
        let st = State { id: id, trans: trs };
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

    pub fn create_nfa(&self, start: StateID, acc: StateID) -> Option<NFA> {
        if self.check_id(start) && self.check_id(acc) {
            Some(NFA { ns: self, start: start, accept: acc })
        }
        else {
            None
        }
    }

    pub fn nfa_epsilon(&mut self) -> NFA {
        let start = self.create_state();
        let acc = self.create_state();

        self.add_transition(start, acc, Label::Epsilon);

        NFA { ns: self, start: start, accept: acc }
    }

    pub fn nfa_symbol(&mut self, symbol: char) -> NFA {
        let start = self.create_state();
        let acc = self.create_state();

        self.add_transition(start, acc, Label::Symbol(symbol));

        NFA { ns: self, start: start, accept: acc }
    }

    pub fn nfa_union(&mut self, n1: NFA, n2: NFA) -> NFA {
        let start = self.create_state();
        self.add_transition(start, n1.start, Label::Epsilon);
        self.add_transition(start, n2.start, Label::Epsilon);

        let acc = self.create_state();
        self.add_transition(n1.accept, acc, Label::Epsilon);
        self.add_transition(n2.accept, acc, Label::Epsilon);

        NFA { ns: self, start: start, accept: acc } 
    }

    pub fn nfa_concat(&mut self, n1: NFA, n2: NFA) -> NFA {
        self.add_transition(n1.accept, n2.start, Label::Epsilon);
        NFA { ns: self, start: n1.start, accept: n2.accept }
    }

    pub fn nfa_star(&mut self, n: NFA) -> NFA {
        let start = self.create_state();
        let acc = self.create_state();

        // transitions connecting old and new start/accept states
        self.add_transition(start, n.start, Label::Epsilon);
        self.add_transition(n.accept, acc, Label::Epsilon);

        // transition connecting start and accept for 0 occurrences
        self.add_transition(start, acc, Label::Epsilon);

        // loop transition for any number of occurrences
        self.add_transition(n.accept, n.start, Label::Epsilon);

        NFA { ns: self, start: start, accept: acc }
    }
}

// NFAs with a single accepting state, which is sufficient 
// for automata generated from regular expressions
struct NFA<'a> {
    ns: &'a NFASet,    // FIX: forces mut borrowing when building NFAs
    start: StateID,
    accept: StateID
}

impl<'a> NFA<'a> {
    fn epsilon_closure(&self, states: Vec<StateID>) -> Vec<StateID> {
        let mut stack : Vec<StateID> = Vec::with_capacity(states.len());

        stack.extend(states.clone());

        while !stack.is_empty() {
            let s = stack.pop().unwrap();
            let st = self.ns.get_state(s).unwrap();
            let ets = st.find_transition(Label::Epsilon);
        }

        states
    }

    pub fn simulate(&self, word: &str) -> bool {
        false
    }
}

#[test]
fn states() {
    let mut ns = NFASet::new();
    let id1 = ns.create_state();
    let id2 = ns.create_state();
    assert_eq!(id1, 0);
    assert_eq!(id2, 1);

    assert!(ns.check_id(id1));
    assert!(ns.check_id(id2));
    assert!(!ns.check_id(100));
}

#[test]
fn add_transition() {
    let mut ns = NFASet::new();
    let id1 = ns.create_state();
    let id2 = ns.create_state();
    let id3 = ns.create_state();
    assert!(ns.add_transition(id1, id2, Label::Epsilon));
    assert!(ns.add_transition(id2, id3, Label::Symbol('a')));

    assert!(!ns.add_transition(id1, 1200, Label::Epsilon));
    assert!(!ns.add_transition(1020, id2, Label::Epsilon));
    assert!(!ns.add_transition(1320, 1200, Label::Epsilon));
}

#[test]
fn transitions() {
    let mut ns = NFASet::new();
    let id1 = ns.create_state();
    let id2 = ns.create_state();
    let id3 = ns.create_state();
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

#[test]
fn epsilon() {
    let mut ns = NFASet::new();
    let n1 = ns.nfa_epsilon();

    let st1 = n1.ns.get_state(n1.start).unwrap();  // FIX
    let t1 = st1.find_transition(Label::Epsilon);
    assert_eq!(t1.len(), 1);

    let st2 = n1.ns.get_state(t1[0].target).unwrap(); // FIX
    let t2 = st2.find_transition(Label::Epsilon);
    assert_eq!(t2.len(), 0);
}
