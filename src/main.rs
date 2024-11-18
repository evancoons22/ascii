//use simple_simplex::NoiseConfig;

//fn main() {
    //let mut seed = 4201339;
    //for _ in 0..10 {
        //let config: NoiseConfig = NoiseConfig::new(
            //3, // Octaves
            //0.01, // X-Frequency
            //0.01, // Y-Frequency
            //0.05, // Amplitude
            //2.5, // Lacunarity
            //0.5, // Gain
            //(0.0, 255.0), // range
            //seed // seed
            //);
        //seed += 1;
        //let size: i32 = 500;
        //let vector: Vec<char> = vec![' ', '.', '-', '=', 'z','X', '#'];
        //config.output(size, &vector)
    //}
//}

use simple_simplex::NoiseConfig;
use std::thread;
use std::time;
use rand;

fn lerp(start: f32, end: f32, factor: f32) -> f32 {
    start + factor * (end - start)
}

fn main() {
    let seed_start = 4202339;
    //let seed_end = 4201340; // Slightly different seed

    //let mut factor = 0.0;
    let delta = 0.005; // Adjust this value to control the speed of morphing
    //let size = (300 as i32, 100 as i32);
    let size =  200;
    //let vector: Vec<char> = vec![' ', '.', '-', '=', 'z', 'X', '#'];
    let vector: Vec<char> = vec![' ', ',', '*', 'x', 'X', '$', '@'];


    //// Random config start
    ////
    ////


    let config_end: NoiseConfig = NoiseConfig::new(
        2, // Octaves
        0.009, // X-Frequency
        0.009, // Y-Frequency
        0.09, // Amplitude
        4.5, // Lacunarity
        0.5, // Gain
        (0.0, 255.0), // range
        seed_start // seed
        );

    //config start defined once
    let mut configstart = config_end;

    loop {
        let mut factor = 0.0;

        let configend: NoiseConfig = NoiseConfig::new(
            rand::random::<i32>() % 10 + 1, // Octaves
            rand::random::<f32>() * 0.009 + 0.001, // X-Frequency
            rand::random::<f32>() * 0.009 + 0.001, // Y-Frequency
            rand::random::<f32>() * 0.09 + 0.01, // Amplitude
            rand::random::<f32>() * 9.0 + 1.0, // Lacunarity
            rand::random::<f32>() * 0.9 + 0.1, // Gain
            (0.0, 255.0), // range
            seed_start // seed
            );

        // println!("starting next iteration");
        thread::sleep(time::Duration::from_millis(100));

        // random config end
        while factor <= 1.0 { 
            //reset the cursor to the top left
            //print!("\x1B[2J\x1B[1;1H");
            // Interpolate between the two noise configurations
            let interpolated_config = interpolate_config(&configstart, &configend, factor);

            // Output the interpolated noise map
            interpolated_config.output(size, &vector);

            factor += delta;
            //sleep for 1 second
            thread::sleep(time::Duration::from_millis(50));
        }
        configstart = configend;
    }
}

// Implement a function to interpolate between two NoiseConfigs
fn interpolate_config(config1: &NoiseConfig, config2: &NoiseConfig, factor: f32) -> NoiseConfig {
    let interpolated_octaves = lerp(config1.octaves as f32, config2.octaves as f32, factor);
    let interpolated_x_frequency = lerp(config1.x_frequency, config2.x_frequency, factor);
    let interpolated_y_frequency = lerp(config1.y_frequency, config2.y_frequency, factor);
    let interpolated_amplitude = lerp(config1.amplitude, config2.amplitude, factor);
    let interpolated_lacunarity = lerp(config1.lacunarity, config2.lacunarity, factor);
    let interpolated_gain = lerp(config1.gain, config2.gain, factor);

    // Assuming range and seed are not interpolated but you could add them if it makes sense
    NoiseConfig::new(
        interpolated_octaves as i32, // Cast back to original type if necessary
        interpolated_x_frequency,
        interpolated_y_frequency,
        interpolated_amplitude,
        interpolated_lacunarity,
        interpolated_gain,
        config1.range, // or some interpolation of config1.range and config2.range
        config1.seed  // or some interpolation of config1.seed and config2.seed
    )
}

