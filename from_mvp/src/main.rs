mod smart_pointers;

fn main() {
    println!("Hello, world!");
    smart_pointers::box_concept::box_concept();
    smart_pointers::deref_trait_intro::deref_trait_intro();
    smart_pointers::drop_trait_intro::drop_intro();
    smart_pointers::rc_intro::rc_intro();
}
