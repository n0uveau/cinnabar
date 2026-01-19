use cinnabar::{Camera, Diff, Extractor, Seeder};
use std::{thread, time::Duration};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut seeder = Seeder::new();

    let mut camera = Camera::new(0)?;
    let extractor = Diff::with_threshold(5_000);

    println!("Collecting entropy from camera...");

    for _ in 0..10 {
        let frames = camera.capture_many(2)?;
        let entropy = extractor.extract(&frames);

        seeder.feed(&entropy);

        thread::sleep(Duration::from_millis(200));
    }

    let seed = seeder.finalize();

    println!("Seed: {:02x?}", seed);

    Ok(())
}
