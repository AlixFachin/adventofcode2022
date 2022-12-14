use std::collections::VecDeque;

// Monkey Rules:

// ONE ROUND
//   Monkey 0 inspects all items (= one monkey's turn)
//      inspects: apply its own operation for the worry level
//      after inspection, before the test: < worry level is divided by 3 and rounded down >
//      tests if the worry level conforms to the tests
//      Throw item to other monkey according to Rules
//      When all the items are inspected, turn goes to next monkey

// When throw to other monkey, item goes to END of the list (FIFO)
// If monkey has no item, its turn ends

// Objective question 1:
// Count the number of times the monkey inspected an item!

const TEST_MODE: bool = false;
const NR_ROUNDS: i32 = 10_000;

enum MonkeyOp {
    Add(i32),
    Square,
    Multiply(i32),
}

// Contains the modulo of the current item worry

struct Monkey {
    number: usize,
    test_mod: i32,
    worry_op: MonkeyOp,
    recipient_if_true: usize,
    recipient_if_false: usize,
}

impl Monkey {
    fn apply_worry(&self, init_worry: &Vec<i32>) -> Vec<i32> {
        match self.worry_op {
            MonkeyOp::Add(x) => {
                // println!("    Worry level increases by {} to {}", x, init_worry + x);
                return init_worry.iter().map(|w| w + x).collect();
            }
            MonkeyOp::Square => {
                // println!(
                //     "    worry level is multiplied by itself to {}",
                //     init_worry * init_worry
                // );
                return init_worry.iter().map(|w| w * w).collect();
            }
            MonkeyOp::Multiply(x) => {
                // println!(
                //     "    Worry level is multiplied by {} to {}",
                //     x,
                //     init_worry * x
                // );
                return init_worry.iter().map(|w| w * x).collect();
            }
        }
    }

    fn get_next_monkey_id(&self, worry_level: &Vec<i32>) -> usize {
        if worry_level[self.number] % self.test_mod == 0 {
            // println!(
            //     "    Current worry level {} is divisible by {}",
            //     worry_level, self.test_mod
            // );
            return self.recipient_if_true;
        }
        // println!(
        //     "    Current worry level {} is not divisible by {}",
        //     worry_level, self.test_mod
        // );
        return self.recipient_if_false;
    }
}

// Struct MonkeyBusiness contains the game state (non mutable)
struct MonkeyBusiness {
    num_monkeys: usize,
    monkey_list: Vec<Monkey>,
    divisor_list: Vec<i32>,
}

// Struct MonkeyData contains the game state (mutable)
struct MonkeyData {
    item_list: Vec<VecDeque<Vec<i32>>>,
    monkey_activity: Vec<usize>,
    divisor_list: Vec<i32>,
}

impl MonkeyBusiness {
    fn init(&mut self) {
        self.monkey_list = Vec::new();
        if TEST_MODE {
            self.num_monkeys = 4;
            self.monkey_list = vec![
                Monkey {
                    number: 0,
                    worry_op: MonkeyOp::Multiply(19),
                    test_mod: 23,
                    recipient_if_true: 2,
                    recipient_if_false: 3,
                },
                Monkey {
                    number: 1,
                    worry_op: MonkeyOp::Add(6),
                    test_mod: 19,
                    recipient_if_true: 2,
                    recipient_if_false: 0,
                },
                Monkey {
                    number: 2,
                    worry_op: MonkeyOp::Square,
                    test_mod: 13,
                    recipient_if_true: 1,
                    recipient_if_false: 3,
                },
                Monkey {
                    number: 3,
                    worry_op: MonkeyOp::Add(3),
                    test_mod: 17,
                    recipient_if_true: 0,
                    recipient_if_false: 1,
                },
            ];
            self.divisor_list = vec![23, 19, 13, 17];
        } else {
            self.num_monkeys = 8;
            self.monkey_list = vec![
                Monkey {
                    number: 0,
                    worry_op: MonkeyOp::Multiply(17),
                    test_mod: 11,
                    recipient_if_true: 2,
                    recipient_if_false: 3,
                },
                Monkey {
                    number: 1,
                    worry_op: MonkeyOp::Add(7),
                    test_mod: 3,
                    recipient_if_true: 6,
                    recipient_if_false: 5,
                },
                Monkey {
                    number: 2,
                    worry_op: MonkeyOp::Square,
                    test_mod: 5,
                    recipient_if_true: 1,
                    recipient_if_false: 7,
                },
                Monkey {
                    number: 3,
                    worry_op: MonkeyOp::Add(1),
                    test_mod: 7,
                    recipient_if_true: 2,
                    recipient_if_false: 7,
                },
                Monkey {
                    number: 4,
                    worry_op: MonkeyOp::Multiply(3),
                    test_mod: 19,
                    recipient_if_true: 0,
                    recipient_if_false: 3,
                },
                Monkey {
                    number: 5,
                    worry_op: MonkeyOp::Add(4),
                    test_mod: 2,
                    recipient_if_true: 6,
                    recipient_if_false: 4,
                },
                Monkey {
                    number: 6,
                    worry_op: MonkeyOp::Add(8),
                    test_mod: 13,
                    recipient_if_true: 4,
                    recipient_if_false: 0,
                },
                Monkey {
                    number: 7,
                    worry_op: MonkeyOp::Add(6),
                    test_mod: 17,
                    recipient_if_true: 1,
                    recipient_if_false: 5,
                },
            ];
            self.divisor_list = vec![11, 3, 5, 7, 19, 2, 13, 17];
        }
    }

    fn iterate_one_round(&self, monkey_data: &mut MonkeyData) {
        for monkey in &self.monkey_list {
            // First, check if the monkey has objects
            // if DEBUG_DISPLAY {
            //     println!("Start of monkey {} turn........", monkey.number);
            // }
            monkey_data.one_monkey_turn(monkey);
        }
    }
}

// helper to create the worry modulo array at the beginning
fn create_modulo_vector(input_array: &Vec<i32>, n: usize) -> VecDeque<Vec<i32>> {
    let mut return_queue: VecDeque<Vec<i32>> = VecDeque::new();
    for worry in input_array {
        let mut worry_vec: Vec<i32> = Vec::new();
        for _i in 0..n {
            worry_vec.push(*worry);
        }
        return_queue.push_back(worry_vec);
    }
    return return_queue;
}

impl MonkeyData {
    fn init(&mut self) {
        if TEST_MODE {
            self.monkey_activity = vec![0, 0, 0, 0];
            self.item_list = vec![
                VecDeque::from([vec![79, 79, 79, 79], vec![98, 98, 98, 98]]),
                VecDeque::from([
                    vec![54, 54, 54, 54],
                    vec![65, 65, 65, 65],
                    vec![75, 75, 75, 75],
                    vec![74, 74, 74, 74],
                ]),
                VecDeque::from([
                    vec![79, 79, 79, 79],
                    vec![60, 60, 60, 60],
                    vec![97, 97, 97, 97],
                ]),
                VecDeque::from([vec![74, 74, 74, 74]]),
            ];
            self.divisor_list = vec![23, 19, 13, 17];
        } else {
            self.monkey_activity = vec![0, 0, 0, 0, 0, 0, 0, 0];
            self.item_list = vec![
                create_modulo_vector(&Vec::from([56, 52, 58, 96, 70, 75, 72]), 8),
                create_modulo_vector(&Vec::from([75, 58, 86, 80, 55, 81]), 8),
                create_modulo_vector(&Vec::from([73, 68, 73, 90]), 8),
                create_modulo_vector(&Vec::from([72, 89, 55, 51, 59]), 8),
                create_modulo_vector(&Vec::from([76, 76, 91]), 8),
                create_modulo_vector(&Vec::from([88]), 8),
                create_modulo_vector(&Vec::from([64, 63, 56, 50, 77, 55, 55, 86]), 8),
                create_modulo_vector(&Vec::from([79, 58]), 8),
            ];
            self.divisor_list = vec![11, 3, 5, 7, 19, 2, 13, 17];
        }
    }

    // Function called to add to the corresponding monkey queue the item with the corresponding worry list
    fn transfer_to_monkey(&mut self, tgt_monkey: usize, worry_level: Vec<i32>) {
        // println!(
        //     "    Item with worry level {} is transferred to monkey {}",
        //     worry_level, tgt_monkey
        // );
        self.item_list[tgt_monkey].push_back(worry_level);
    }

    // Function which pops the next item from the monkey list and returns the corresponding worry level
    fn get_next_item(&mut self, tgt_monkey: usize) -> Option<Vec<i32>> {
        return self.item_list[tgt_monkey].pop_front();
    }

    fn one_monkey_turn(&mut self, monkey: &Monkey) {
        while let Some(worry_item) = self.get_next_item(monkey.number) {
            self.monkey_activity[monkey.number] = self.monkey_activity[monkey.number] + 1;
            // println!(
            //     "  Monkey inspects an item with a worry level of {}",
            //     worry_item
            // );
            let mut current_worry = worry_item.clone();
            current_worry = monkey.apply_worry(&current_worry);

            // Reducing size using modulos
            for i in 0..current_worry.len() {
                current_worry[i] = current_worry[i] % self.divisor_list[i];
            }

            let next_monkey = monkey.get_next_monkey_id(&current_worry);
            self.transfer_to_monkey(next_monkey, current_worry);
        }
    }

    fn display_activity(&self) {
        let mut i = 0;
        for activity in &self.monkey_activity {
            println!("Monkey {} inspected items {} times", i, activity);
            i += 1;
        }
    }
}

pub fn solve() {
    let mut game = MonkeyBusiness {
        num_monkeys: 0,
        monkey_list: Vec::new(),
        divisor_list: Vec::new(),
    };
    let mut monkey_data = MonkeyData {
        monkey_activity: Vec::new(),
        item_list: Vec::new(),
        divisor_list: Vec::new(),
    };

    game.init();
    monkey_data.init();

    for round in 0..NR_ROUNDS {
        println!("Starting round {}", round);
        game.iterate_one_round(&mut monkey_data);
        monkey_data.display_activity();
    }

    monkey_data.display_activity();
}
