pub mod random_border;
pub mod model_generation;

// Default difficulty is max. You can increase value, but this willn't make next generation harder.

const RANDOM_BORDER_DIFFICULTY: usize = 10; // Default: 10
const RANDOM_BORDER_TESTS: usize = 3; // Default: 3

const MODEL_GENERATION_DIFFICULTY: usize = 4; // Default: 4
const MODEL_GENERATION_TESTS: usize = 3; // Default: 3

fn main() {
    println!("Running benchmarks...");
    let random_border_dur = random_border::bench(RANDOM_BORDER_DIFFICULTY, RANDOM_BORDER_TESTS);

    let (model_gen_all, model_gen_full, model_gen_mask, model_gen_model) = 
        model_generation::bench(MODEL_GENERATION_DIFFICULTY, MODEL_GENERATION_TESTS);

    println!("\n\nRandom border average duration
        (difficulty: {RANDOM_BORDER_DIFFICULTY}, tests: {RANDOM_BORDER_TESTS}): {:.2?}\n\n", random_border_dur);

    println!("Model generation average duration
        (difficulty: {MODEL_GENERATION_DIFFICULTY}, tests: {MODEL_GENERATION_TESTS}):
        All_time: {:.2?}, Full model: {:.2?}, : Mask only: {:.2?}, Model only: {:.2?}\n\n",
        model_gen_all, model_gen_full, model_gen_mask, model_gen_model);
}
