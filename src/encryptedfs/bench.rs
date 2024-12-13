#[allow(unused_imports)]
use std::str::FromStr;

#[allow(unused_imports)]
use rand::Rng;
#[allow(unused_imports)]
use shush_rs::SecretString;

#[allow(unused_imports)]
use crate::encryptedfs::{DirectoryEntry, DirectoryEntryPlus, FileType, ROOT_INODE};
#[allow(unused_imports)]
use crate::test_common::{create_attr, get_fs};
#[allow(unused_imports)]
use crate::test_common::block_on;   
#[allow(unused_imports)]
use crate::{async_util, test_common};

#[allow(unused_imports)]
use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[allow(dead_code)]
fn bench_create(c: &mut Criterion) {
    c.bench_function("bench_create", |b| {
        block_on(async {
            let fs = get_fs().await;

            let mut i = 1;
            b.iter(|| {
                block_on(async {
                    let test_file = SecretString::from_str(&format!("test-file-{}", i)).unwrap();
                    let _ = fs
                        .create(
                            ROOT_INODE,
                            &test_file,
                            create_attr(FileType::RegularFile),
                            false,
                            false,
                        )
                        .await
                        .unwrap();
                    i += 1;
                }, 1);
                black_box(());
            });
        }, 1);
    });
}

#[allow(dead_code)]
fn bench_exists_by_name(c: &mut Criterion) {
    c.bench_function("exists_by_name", |b| {
        block_on(async {
            let fs = get_fs().await;
            let mut rnd = rand::thread_rng();

            b.iter(|| {
                block_on(async {
                    let _ = fs
                        .exists_by_name(
                            ROOT_INODE,
                            &SecretString::from_str(&format!("test-file-{}", rnd.gen_range(1..100)))
                                .unwrap(),
                        )
                        .unwrap();
                }, 1);
                black_box(());
            });
        },1);
    });
}

#[allow(dead_code)]
fn bench_find_by_name(c: &mut Criterion) {
    c.bench_function("bench_find_by_name", |b| {
        block_on(async {
            let fs = get_fs().await;

            // Setup: Pre-create files
            for i in 0..100 {
                let test_file = SecretString::from_str(&format!("test-file-{}", i)).unwrap();
                let _ = fs
                    .create(
                        ROOT_INODE,
                        &test_file,
                        create_attr(FileType::RegularFile),
                        false,
                        false,
                    )
                    .await
                    .unwrap();
            }

            let mut rnd = rand::thread_rng();
            b.iter(|| {
                block_on(async {
                    let _ = fs
                        .find_by_name(
                            ROOT_INODE,
                            &SecretString::from_str(&format!("test-file-{}", rnd.gen_range(1..100)))
                                .unwrap(),
                        )
                        .await
                        .unwrap();
                }, 1);
                black_box(());
            });
        }, 1);
    });
}

#[allow(dead_code)]
fn bench_read_dir(c: &mut Criterion) {
    c.bench_function("bench_read_dir", |b| {
        block_on(async {
            let fs = get_fs().await;

            // Setup: Pre-create files
            for i in 0..100 {
                let test_file = SecretString::from_str(&format!("test-file-{}", i)).unwrap();
                let _ = fs
                    .create(
                        ROOT_INODE,
                        &test_file,
                        create_attr(FileType::RegularFile),
                        false,
                        false,
                    )
                    .await
                    .unwrap();
            }

            b.iter(|| {
                block_on(async {
                    let iter = fs.read_dir(ROOT_INODE).await.unwrap();
                    let vec: Vec<DirectoryEntry> = iter.map(|e| e.unwrap()).collect();
                    black_box(vec);
                }, 1);
                black_box(());
            });
        }, 1);
    });
}

#[allow(dead_code)]
fn bench_read_dir_plus(c: &mut Criterion) {
    c.bench_function("bench_read_dir_plus", |b| {
        block_on(async {
            let fs = get_fs().await;

            // Setup: Pre-create files
            for i in 0..100 {
                let test_file = SecretString::from_str(&format!("test-file-{}", i)).unwrap();
                let _ = fs
                    .create(
                        ROOT_INODE,
                        &test_file,
                        create_attr(FileType::RegularFile),
                        false,
                        false,
                    )
                    .await
                    .unwrap();
            }

            b.iter(|| {
                block_on(async {
                    let iter = fs.read_dir_plus(ROOT_INODE).await.unwrap();
                    let vec: Vec<DirectoryEntryPlus> = iter.map(|e| e.unwrap()).collect();
                    black_box(vec);
                }, 1);
                black_box(());
            });
        }, 1);
    });
}

criterion_group!(
    benches,
    bench_create,
    bench_exists_by_name,
    bench_find_by_name,
    bench_read_dir,
    bench_read_dir_plus
);
criterion_main!(benches);
