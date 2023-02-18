// pll_simple_example.rs : simulation of a phase-locked loop in 50 lines
// Ported from https://liquidsdr.org/blog/pll-simple-howto/
use num::Complex;

fn main() {
    // parameters and simulation options
    let mut phase_in: f32 = 3.0;   // carrier phase offset
    let frequency_in: f32 = -0.20; // carrier frequency offset
    let alpha: f32        = 0.05;  // phase adjustment factor
    let n: u16            = 400;   // number of samples

    // initialize states
    let beta  = 0.5*alpha*alpha;    // frequency adjustment factor
    let mut phase_out     = 0.0;    // output signal phase
    let mut frequency_out = 0.0;    // outpus signal frequency

    // print line legend to standard output
    println!("# {:>6} {:>12} {:>12} {:>12} {:>12} {:>12}",
        "index",
        "real(in)",
        "imag(in)",
        "real(out)",
        "imag(out)",
        "error",
    );

    // run basic simulation
    for i in 0..n {
        let signal_in: Complex<f32> = (Complex::i() * phase_in).exp();
        let signal_out: Complex<f32> = (Complex::i() * phase_out).exp();

        // compute phase error estimate
        let phase_error = (signal_in * signal_out.conj()).arg();

        // print results to standard out for plotting
        println!("# {:>6} {:>12} {:>12} {:>12} {:>12} {:>12}",
            i,
            signal_in.re, signal_in.im,
            signal_out.re, signal_out.im,
            phase_error,
        );

        // apply loop filter and correct output phase and frequency
        phase_out     += alpha * phase_error; // adjust phase
        frequency_out +=  beta * phase_error; // adjust frequency

        // increment input and output phase values
        phase_in  += frequency_in;
        phase_out += frequency_out;
    }
}
