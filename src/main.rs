/// This program analyzes the RNG (Random Number Generator) of Pikmin 2.
///
/// Thanks to @APerson13 for this decompilation.
/// https://discord.com/channels/177495849100640256/698992259038838864/906737629616279612
///
/// ```C
/// s16 rand(void)
/// {
///     next = next * 0x41c64e6d + 0x3039;
///     return (s16)((u16)((u32)next >> 0x10) & 0x7fff);
///     /*
///   .loc_0x0:
///     lis       r3, 0x41C6
///     lwz       r4, -0x7DD8(r13)
///     addi      r0, r3, 0x4E6D
///     mullw     r3, r4, r0
///     addi      r0, r3, 0x3039
///     stw       r0, -0x7DD8(r13)
///     rlwinm    r3,r0,16,17,31
///     blr
///   */
/// }
/// ```

/// Results:
/// The internal seed value (`next`) reaches all possible 32bit values,
/// and none is reached twice, before it loops over those again.
/// This is the best behaviour for this simple type of RNG.
///
/// The return value is a 15-bit result though, which is a bit strange.
/// TODO analyze if it has some bias.

/// Run this program with `cargo run --release`
fn main() {
    const TWO_POW_32: usize = 2_usize.pow(32);
    // Allocate a big array with a counter for every possible 32bit value.
    let mut counts = vec![0_u8; TWO_POW_32];

    // I don't know with what the seed is initialized.
    // Doesn't matter though, since all values are reached.
    let mut rng_seed = 0_u32;
    counts[0] += 1;

    println!("Calculating all RNG seed values. This can take a few minutes...");
    // Call the RNG in a loop, to simulate the game calling it many times.
    for i in 1..TWO_POW_32 {
        // We want 2's compliment arithmetic.
        rng_seed = rng_next_seed(rng_seed);
        counts[rng_seed as usize] += 1;

        if counts[rng_seed as usize] >= 2 {
            // found a cycle
            println!("Found a cycle on seed 0x{:X}, index 0x{:X}", rng_seed, i);
        }
    }
    println!("Finished calculating all seeds. Searching for unreached seeds...");

    // See if any seed was reached twice.
    for i in 0..TWO_POW_32 {
        if counts[i] == 0 {
            println!("Seed 0x{:X} not reached", i);
        }
    }
    println!("Finished searching for unreached seeds.");
}

/// This is how Pikmin 2 calculates the next RNG seed.
#[inline(always)]
fn rng_next_seed(seed: u32) -> u32 {
    seed.wrapping_mul(0x41c64e6d).wrapping_add(0x3039)
}


