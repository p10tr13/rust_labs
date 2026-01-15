use std::collections::HashMap;
use std::hash::Hash;

// Ex. 1
#[macro_export]
macro_rules! string {
    ($x:expr) => {
        String::from($x)
    };
}

// Ex. 2
pub trait StateMachine<S> {
    fn step(&self, state: S) -> Option<S>;
}

// Ex. 3
#[macro_export]
macro_rules! impl_state_machine {
    ( $name:ident, [ $( $from:tt -> $to:tt; )* ]) => {
        pub struct $name {
            transition: std::collections::HashMap<i32, i32>,
        }

        impl $name {
            #[allow(dead_code)]
            pub fn new() -> Self {
                let mut map = std::collections::HashMap::new();

                macro_rules! parse_dest {
                    (END) => { -1 };
                    ($x:expr) => { $x };
                }

                $(
                map.insert($from, parse_dest!($to));
                )*

                Self{transition: map}
            }
        }

        impl StateMachine<i32> for $name {
            fn step(&self, state: i32) -> Option<i32> {
                match self.transition.get(&state) {
                    Some(&dest) if dest == -1 => None,
                    Some(&dest) => Some(dest),
                    None => None
                }
            }
        }
    };
}

// Ex. 4
impl<S> StateMachine<S> for HashMap<S, S>
where
    S: Clone + Eq + Hash,
{
    fn step(&self, state: S) -> Option<S> {
        self.get(&state).cloned()
    }
}

pub fn join_machines<M1, M2, S>(x: M1, y: M2) -> Vec<Box<dyn StateMachine<S>>>
where
    M1: StateMachine<S> + 'static,
    M2: StateMachine<S> + 'static,
{
    vec![Box::new(x), Box::new(y)]
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ex. 1
    #[test]
    fn test_string_macro_literal() {
        let s = string!("hello");
        assert_eq!(s, String::from("hello"));
    }

    #[test]
    fn test_string_macro_variable() {
        let text_slice = "slice";
        let s = string!(text_slice);
        assert_eq!(s, "slice".to_string());
    }

    #[test]
    fn test_empty_string() {
        let s = string!("");
        assert!(s.is_empty());
    }

    // Ex. 2 i 3
    impl_state_machine!(MyMachine, [
        1 -> 3;
        2 -> 3;
        3 -> 4;
        4 -> END;
    ]);

    #[test]
    fn test_generated_machine_path_1() {
        let machine = MyMachine::new();
        let mut state = 1;

        // 1 -> 3
        let next = machine.step(state);
        assert_eq!(next, Some(3));
        state = next.unwrap();

        // 3 -> 4
        let next = machine.step(state);
        assert_eq!(next, Some(4));
        state = next.unwrap();

        // 4 -> END (None)
        let next = machine.step(state);
        assert_eq!(next, None);
    }

    #[test]
    fn test_invalid_state() {
        let machine = MyMachine::new();
        assert_eq!(machine.step(99), None);
    }

    // Ex. 4
    #[test]
    fn test_hashmap_machine_i32() {
        let mut map = HashMap::new();
        map.insert(1, 2);
        map.insert(2, 3);
        // 3 not in map, so 3 -> None

        assert_eq!(map.step(1), Some(2));
        assert_eq!(map.step(2), Some(3));
        assert_eq!(map.step(3), None);
    }

    #[test]
    fn test_hashmap_machine_string() {
        let mut map = HashMap::new();
        map.insert(string!("Start"), string!("Stop"));

        // Step 1: Start -> Stop
        let next = map.step(string!("Start"));
        assert_eq!(next, Some(string!("Stop")));

        // Step 2: Stop -> None (cuz "Stop" is not key in map)
        let next = map.step(next.unwrap());
        assert_eq!(next, None);
    }

    // Ex. 5
    #[test]
    fn test_join_machines() {
        let m1 = MyMachine::new();
        let mut m2 = HashMap::new();

        m2.insert(1, 99);

        // Joining
        let machines = join_machines(m1, m2);

        // Checking weather we have 2 machines
        assert_eq!(machines.len(), 2);

        // machines[0] to MyMachine: 1 -> 3
        assert_eq!(machines[0].step(1), Some(3));

        // machines[1] to HashMap: 1 -> 99
        assert_eq!(machines[1].step(1), Some(99));
    }
}
