use super::random::Random;

#[derive(Default)]
pub struct GuitarString {
    note_attenuation: f32,
    // deque: ArrayDeque<f32>, TODO: uncomment this
}

impl GuitarString {
    pub fn new(sample_rate: u32, note_atten: f32, freq: u32) -> Self {
        let deque_len = sample_rate / freq;
        unimplemented!(); // TODO: create a GuitarString with a buffer filled with 0s
    }

    /* Pluck the guitar string by replacing the buffer with white noise. */
    pub fn pluck(&mut self, rand: &mut Random) {

        // example usage of rand
        let val = (rand.next_f64() - 0.5) as f32;
        // "as" is ok because we want to lose precision

        unimplemented!();
        // TODO: Dequeue everything in buffer, and replace with random numbers
        //       between -0.5 and 0.5.
        //
        //       Make sure that your random numbers are different from each
        //       other. This does not mean that you need to check that the numbers
        //       are different from each other. It means you should repeatedly call
        //       rand.next_f64() - 0.5 to generate new random numbers for each array index.
    }

    /* Advance the simulation by performing one iteration of the Karplus-Strong algorithm:
        - pop the first sample off of the queue
        - calculate the next sample, and push it into the queue
        - return the popped sample
    */
    pub fn advance(&mut self) -> f32 {
        unimplemented!(); // TODO: code
    }
}
