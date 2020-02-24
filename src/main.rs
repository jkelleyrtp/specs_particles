#[cfg(not(target_arch = "wasm32"))]
use {
    ocl,
    ocl::builders::ContextProperties,
    ocl::enums::ArgVal,
    ocl::{core, flags},
};


use rand::thread_rng;
use specs::prelude::*;
use specs_particles::{
    components::{Position, Velocity},
    simulation,
};
use std::ffi::CString;

fn run_test(num_agents: i64, num_steps: i64) {
    let mut my_sim = simulation::ElectronSim::new();

    let t_start = std::time::Instant::now();

    for _ in 0..num_agents {
        my_sim
            .world
            .create_entity()
            .with(Velocity { x: 0.0, y: 0.0 })
            .with(Position { x: 0.0, y: 0.0 })
            .build();
    }

    println!(
        "Time taken to generate {} agents: {:?}",
        num_agents,
        std::time::Instant::now() - t_start
    );

    let t_start = std::time::Instant::now();
    for _ in 0..num_steps {
        my_sim.dispatcher.dispatch(&mut my_sim.world);
    }
    println!(
        "Time taken to iterate {} steps for {} agents: {:?}",
        num_steps,
        num_agents,
        std::time::Instant::now() - t_start
    );
    println!("\n");
}

fn main() {
    println!("Running performance tests...");

    // run_test(10, 10);
    // run_test(100, 10);
    // run_test(1000, 10);
    // run_test(10000, 10);
    // run_test(100000, 10);

    // run_test(1000, 10);
    // run_test(1000, 100);
    // run_test(1000, 1000);
    // run_test(1000, 10000);
    // run_test(1000, 100000);

    run_test(10000, 100000);

    #[cfg(not(target_arch = "wasm32"))]
    trivial_cored().unwrap();
}

#[cfg(not(target_arch = "wasm32"))]
fn trivial_cored() -> ocl::core::Result<()> {
    let src = r#"
        __kernel void add(__global float* buffer, float scalar) {
            buffer[get_global_id(0)] += scalar;
        }
    "#;

    // (1) Define which platform and device(s) to use. Create a context,
    // queue, and program then define some dims..
    let platform_id = core::default_platform()?;
    let device_ids = core::get_device_ids(&platform_id, None, None)?;
    let device_id = device_ids[0];
    let context_properties = ContextProperties::new().platform(platform_id);
    let context = core::create_context(Some(&context_properties), &[device_id], None, None)?;
    let src_cstring = CString::new(src)?;
    let program = core::create_program_with_source(&context, &[src_cstring])?;
    core::build_program(&program, Some(&[device_id]), &CString::new("")?, None, None)?;
    let queue = core::create_command_queue(&context, &device_id, None)?;
    let dims = [1 << 20, 1, 1];

    // (2) Create a `Buffer`:
    let mut vec = vec![0.0f32; dims[0]];
    let buffer = unsafe {
        core::create_buffer(
            &context,
            flags::MEM_READ_WRITE | flags::MEM_COPY_HOST_PTR,
            dims[0],
            Some(&vec),
        )?
    };

    // (3) Create a kernel with arguments matching those in the source above:
    let kernel = core::create_kernel(&program, "add")?;
    core::set_kernel_arg(&kernel, 0, ArgVal::mem(&buffer))?;
    core::set_kernel_arg(&kernel, 1, ArgVal::scalar(&10.0f32))?;

    // Do a queue
    unsafe {
        ocl::core::enqueue_kernel(
            &queue,
            &kernel,
            1,
            None,
            &dims,
            None,
            None::<core::Event>,
            None::<&mut core::Event>,
        )?;
    }

    // Do a read
    unsafe {
        ocl::core::enqueue_read_buffer(
            &queue,
            &buffer,
            true,
            0,
            &mut vec,
            None::<core::Event>,
            None::<&mut core::Event>,
        )?;
    }
    println!("The value at index [{}] is now '{}'!", 200007, vec[200007]);

    // (4) Run the kernel:
    let t_start = std::time::Instant::now();
    let num_steps = 1000;
    for _ in 0..num_steps {
        unsafe {
            core::enqueue_kernel(
                &queue,
                &kernel,
                1,
                None,
                &dims,
                None,
                None::<core::Event>,
                None::<&mut core::Event>,
            )?;
        }
    }
    println!(
        "Took {:?} to process {} steps for 1048576 agents",
        std::time::Instant::now() - t_start,
        num_steps
    );
    // (5) Read results from the device into a vector:

    let t_start = std::time::Instant::now();
    unsafe {
        core::enqueue_read_buffer(
            &queue,
            &buffer,
            true,
            0,
            &mut vec,
            None::<core::Event>,
            None::<&mut core::Event>,
        )?;
    }
    println!(
        "Took {:?} to copy off the entries",
        std::time::Instant::now() - t_start
    );

    // Print an element:
    println!("The value at index [{}] is now '{}'!", 200007, vec[200007]);
    Ok(())
}
