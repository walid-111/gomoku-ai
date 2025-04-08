#![feature(portable_simd)]

use std::simd::{f32x4, i32x4, i16x8};
use std::time::Instant;
use std::hint::black_box;

fn test_adder_basic(target: &mut [f32; 12], offset: usize, source: &[f32; 8]) {
    for i in 0..8 {
        target[offset + i] += source[i];
    }
}

fn test_adder_simd(target: &mut [f32; 12], offset: usize, source_chunk: &(f32x4, f32x4)) {
    let mut target_chunk = f32x4::from_slice(&target[offset..offset + 4]);
    target_chunk += source_chunk.0;
    // target[offset..offset + 4].copy_from_slice(&target_chunk.to_array());
    target_chunk.copy_to_slice(&mut target[offset..offset + 4]);
    let mut target_chunk = f32x4::from_slice(&target[offset + 4..offset + 8]);
    target_chunk += source_chunk.1;
    // target[offset + 4..offset + 8].copy_from_slice(&target_chunk.to_array());
    target_chunk.copy_to_slice(&mut target[offset + 4..offset + 8]);
}

fn test_adder_basic_i32(target: &mut [i32; 12], offset: usize, source: &[i32; 8]) {
	for i in 0..8 {
		target[offset + i] += source[i];
	}
}

fn test_adder_simd_i32(target: &mut [i32; 12], offset: usize, source_chunk: &(i32x4, i32x4)) {
	let mut target_chunk = i32x4::from_slice(&target[offset..offset + 4]);
	target_chunk += source_chunk.0;
	target_chunk.copy_to_slice(&mut target[offset..offset + 4]);
	let mut target_chunk = i32x4::from_slice(&target[offset + 4..offset + 8]);
	target_chunk += source_chunk.1;
	target_chunk.copy_to_slice(&mut target[offset + 4..offset + 8]);
}

fn test_adder_basic_i16(target: &mut [i16; 12], offset: usize, source: &[i16; 8]) {
	for i in 0..8 {
		target[offset + i] += source[i];
	}
}

fn test_adder_simd_i16(target: &mut [i16; 12], offset: usize, source_chunk: &i16x8) {
	// let mut target_chunk = i16x8::from_slice(&target[offset..offset + 8]);
	// target_chunk += source_chunk;
	// target[offset..offset + 8].copy_from_slice(&target_chunk.to_array());
	let mut target_chunk = i16x8::from_slice(&target[offset..offset + 8]);
	target_chunk += source_chunk;
	target_chunk.copy_to_slice(&mut target[offset..offset + 8]);
}

fn test_adder_noop() {}

fn main() {
    let mut target = [0.0f32; 12];
    
    let data: [f32; 8] = [7.0, 2.0, 5.0, 0.0, 1.0, 3.0, 1.0, 5.0];
    let now = Instant::now();
    for _ in 0..10_000 {
        black_box(test_adder_basic(&mut target, 1, &data));
    }
    println!("basic: {:.2?}", now.elapsed());
    
    let data = (f32x4::from_slice(&data[0..4]), f32x4::from_slice(&data[4..8]));
    let now = Instant::now();
    for _ in 0..10_000 {
        black_box(test_adder_simd(&mut target, 1, &data));
    }
    println!("simd: {:.2?}", now.elapsed());
    
    
    
    println!("{:?}", target);
    
    let mut target = [0; 12];
    let data: [i32; 8] = [7, 2, 5, 0, 1, 3, 1, 5];
    let now = Instant::now();
    for _ in 0..10_000 {
		black_box(test_adder_basic_i32(&mut target, 1, &data));
	}
	println!("basic i32: {:.2?}", now.elapsed());
	
	let data = (i32x4::from_slice(&data[0..4]), i32x4::from_slice(&data[4..8]));
	let now = Instant::now();
	for _ in 0..10_000 {
		black_box(test_adder_simd_i32(&mut target, 1, &data));
	}
	println!("simd i32: {:.2?}", now.elapsed());
	
	
	
	
	let mut target = [0; 12];
	let data: [i16; 8] = [7, 2, 5, 0, 1, 3, 1, 5];
	let now = Instant::now();
	for _ in 0..10_000 {
		black_box(test_adder_basic_i16(&mut target, 1, &data));
	}
	println!("basic i16: {:.2?}", now.elapsed());
	
	let data = i16x8::from_slice(&data);
	let now = Instant::now();
	for _ in 0..10_000 {
		black_box(test_adder_simd_i16(&mut target, 1, &data));
	}
	println!("simd i16: {:.2?}", now.elapsed());
	
	let now = Instant::now();
	for _ in 0..10_000 {
		black_box(test_adder_noop());
	}
	println!("noop: {:.2?}", now.elapsed());
    
    println!("{:?}", target);
}