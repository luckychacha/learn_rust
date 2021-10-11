mod smart_pointers;
pub mod no_fear_concurrent_and_parallel;

fn main() {
    // smart_pointers::box_concept::box_concept();
    // smart_pointers::deref_trait_intro::deref_trait_intro();
    // smart_pointers::drop_trait_intro::drop_intro();
    // smart_pointers::rc_intro::rc_intro();
    // smart_pointers::ref_cell_intro::ref_cell_concept();
    // smart_pointers::cycle_reference_cause_overflow::cycle_reference_cause_overflow();

    // no_fear_concurrent_and_parallel::thread_spawn_init::thread_spawn_init();
    // no_fear_concurrent_and_parallel::thread_spawn_with_move::move_demo();
    // no_fear_concurrent_and_parallel::cross_thread_send_message::channel_demo();
    no_fear_concurrent_and_parallel::share_status_concurrency::mutext_demo();
}
