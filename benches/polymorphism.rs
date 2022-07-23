use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_polymorphism::{print_area_where, Circle, Rectangle, Shape, print_area_impl, print_area_simplified, print_area_dyn};

#[allow(unused)]
fn benchmark_where(c: &mut Criterion) {
    let circle = Circle::new();
    let rectangle = Rectangle::new();
    let dyn_circle: &dyn Shape = &Circle::new();
    let dyn_rectangle: &dyn Shape = &Rectangle::new();
    let boxed_circle = Box::new(Circle::new());
    let boxed_rectangle = Box::new(Rectangle::new());
    let boxed_dyn_circle: Box<dyn Shape> = Box::new(Circle::new());
    let boxed_dyn_rectangle: Box<dyn Shape> = Box::new(Rectangle::new());
    
    c.bench_function("normal where", |b| b.iter(|| {
        print_area_where(black_box(&circle));
        print_area_where(black_box(&rectangle));
    }));

    c.bench_function("dyn where", |b| b.iter(|| {
        print_area_where(black_box(dyn_circle));
        print_area_where(black_box(dyn_rectangle));
    }));

    // c.bench_function("boxed normal where", |b| b.iter(|| {
    //     print_area_where(black_box(&*boxed_circle));
    //     print_area_where(black_box(&*boxed_rectangle));
    // }));

    // c.bench_function("boxed dyn where", |b| b.iter(|| {
    //     print_area_where(black_box(&*boxed_dyn_circle));
    //     print_area_where(black_box(&*boxed_dyn_rectangle));
    // }));
}

#[allow(unused)]
fn benchmark_simplified(c: &mut Criterion) {
    let circle = Circle::new();
    let rectangle = Rectangle::new();
    let dyn_circle: &dyn Shape = &Circle::new();
    let dyn_rectangle: &dyn Shape = &Rectangle::new();
    let boxed_circle = Box::new(Circle::new());
    let boxed_rectangle = Box::new(Rectangle::new());
    let boxed_dyn_circle: Box<dyn Shape> = Box::new(Circle::new());
    let boxed_dyn_rectangle: Box<dyn Shape> = Box::new(Rectangle::new());
    
    c.bench_function("normal simplified", |b| b.iter(|| {
        print_area_simplified(black_box(&circle));
        print_area_simplified(black_box(&rectangle));
    }));

    c.bench_function("dyn simplified", |b| b.iter(|| {
        print_area_simplified(black_box(dyn_circle));
        print_area_simplified(black_box(dyn_rectangle));
    }));

    // c.bench_function("boxed normal simplified", |b| b.iter(|| {
    //     print_area_simplified(black_box(&*boxed_circle));
    //     print_area_simplified(black_box(&*boxed_rectangle));
    // }));

    // c.bench_function("boxed dyn simplified", |b| b.iter(|| {
    //     print_area_simplified(black_box(&*boxed_dyn_circle));
    //     print_area_simplified(black_box(&*boxed_dyn_rectangle));
    // }));
}

#[allow(unused)]
fn benchmark_impl(c: &mut Criterion) {
    let circle = Circle::new();
    let rectangle = Rectangle::new();
    let dyn_circle: &dyn Shape = &Circle::new();
    let dyn_rectangle: &dyn Shape = &Rectangle::new();
    let boxed_circle = Box::new(Circle::new());
    let boxed_rectangle = Box::new(Rectangle::new());
    let boxed_dyn_circle: Box<dyn Shape> = Box::new(Circle::new());
    let boxed_dyn_rectangle: Box<dyn Shape> = Box::new(Rectangle::new());
    
    c.bench_function("normal impl", |b| b.iter(|| {
        print_area_impl(black_box(&circle));
        print_area_impl(black_box(&rectangle));
    }));

    c.bench_function("dyn impl", |b| b.iter(|| {
        print_area_impl(black_box(dyn_circle));
        print_area_impl(black_box(dyn_rectangle));
    }));

    // c.bench_function("boxed normal impl", |b| b.iter(|| {
    //     print_area_impl(black_box(&*boxed_circle));
    //     print_area_impl(black_box(&*boxed_rectangle));
    // }));

    // c.bench_function("boxed dyn impl", |b| b.iter(|| {
    //     print_area_impl(black_box(&*boxed_dyn_circle));
    //     print_area_impl(black_box(&*boxed_dyn_rectangle));
    // }));
}

#[allow(unused)]
fn benchmark_dyn(c: &mut Criterion) {
    let circle = Circle::new();
    let rectangle = Rectangle::new();
    let dyn_circle: &dyn Shape = &Circle::new();
    let dyn_rectangle: &dyn Shape = &Rectangle::new();
    let boxed_circle = Box::new(Circle::new());
    let boxed_rectangle = Box::new(Rectangle::new());
    let boxed_dyn_circle: Box<dyn Shape> = Box::new(Circle::new());
    let boxed_dyn_rectangle: Box<dyn Shape> = Box::new(Rectangle::new());
    
    c.bench_function("normal dyn", |b| b.iter(|| {
        print_area_dyn(black_box(&circle));
        print_area_dyn(black_box(&rectangle));
    }));

    c.bench_function("dyn dyn", |b| b.iter(|| {
        print_area_dyn(black_box(dyn_circle));
        print_area_dyn(black_box(dyn_rectangle));
    }));

    // c.bench_function("boxed normal dyn", |b| b.iter(|| {
    //     print_area_dyn(black_box(&*boxed_circle));
    //     print_area_dyn(black_box(&*boxed_rectangle));
    // }));

    // c.bench_function("boxed dyn dyn", |b| b.iter(|| {
    //     print_area_dyn(black_box(&*boxed_dyn_circle));
    //     print_area_dyn(black_box(&*boxed_dyn_rectangle));
    // }));
}

criterion_group!(benches, benchmark_impl, benchmark_dyn);
criterion_main!(benches);