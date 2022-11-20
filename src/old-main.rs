/*- Global allowings -*/
#![allow(
    dead_code
)]

/*- Imports -*/
mod challenges;
use challenges::{ Node, ARM_GEN_PROBABILITY, MAX_TREE_DEPTH, LLNode };
use rand::Rng;

/*- Initialize -*/
fn main() -> () {
    println!("{:?}", challenges::how_much_is_true(vec![false, true, true, false, true]));
    println!("{:?}", challenges::find_seven(vec![1, 125, 123, 654, 1471]));
    println!("{:?}", challenges::find_boomerangs(vec![1, 125, 1, 125, 1471]));
    println!("{:?}", challenges::track_robot(vec![1, 2, 3, 4, 5, 6, 7, 8]));
    let zeroes = &mut vec![1, 6, 2, 0, 2, 0, 6, 8, 1, 0];
    challenges::move_zeroes(zeroes);
    println!("{:?}", zeroes);
    println!("{:?}", challenges::two_powers_of_two(2048));
    println!("{:?}", challenges::three_sum(&vec![1, 2, 34, 5, 7, 8, 1, -1, 0]));
    println!("{:?}", challenges::minesweeper_grid(vec![
        vec!["-", "-", "-", "-", "-"],
        vec!["-", "-", "-", "#", "-"],
        vec!["-", "-", "#", "-", "-"],
        vec!["-", "-", "-", "-", "-"],
        vec!["-", "-", "-", "-", "-"],
    ]));
    /*- Create some branches -*/
    let node = Node::new(generate_node(ARM_GEN_PROBABILITY, 0), generate_node(ARM_GEN_PROBABILITY, 0), 0);

    /*- Find item -*/
    let search = Node::search(&Box::new(node), 2);
    match search {
        Some(value) => println!("FOUND {:?}", value),
        None => println!("Not found"),
    };
    challenges::find_middle(
        LLNode::new(0,
            Some(LLNode::new(2,
                Some(LLNode::new(4,
                    Some(LLNode::new(9,
                        None
                    ))
                ))
            ))
        )
    );

}

pub fn generate_node(probability:f64, index:u32) -> Option<Box<Node>> {
    if index > MAX_TREE_DEPTH { return None };

    /*- Create nodes -*/
    let rand_bool = rand::thread_rng().gen_bool(probability);
    if rand_bool {
        Some(Box::new(Node::new(
            generate_node(probability, index+1),
            generate_node(probability, index+1),
            rand::thread_rng().gen_range(-5..5)
        )))
    } else { None }
}