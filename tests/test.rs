extern crate rusqlite;
extern crate r2d2;
extern crate r2d2_sqlite;
extern crate tempdir;

use std::sync::mpsc;
use std::thread;

use r2d2::ManageConnection;
use tempdir::TempDir;

use r2d2_sqlite::SqliteConnectionManager;

#[test]
fn test_basic() {
    let manager = SqliteConnectionManager::new(":memory:").unwrap();
    let config = r2d2::Config::builder().pool_size(2).build();
    let pool = r2d2::Pool::new(config, manager).unwrap();

    let (s1, r1) = mpsc::channel();
    let (s2, r2) = mpsc::channel();

    let pool1 = pool.clone();
    let t1 = thread::spawn(move || {
        let conn = pool1.get().unwrap();
        s1.send(()).unwrap();
        r2.recv().unwrap();
        drop(conn);
    });

    let pool2 = pool.clone();
    let t2 = thread::spawn(move || {
        let conn = pool2.get().unwrap();
        s2.send(()).unwrap();
        r1.recv().unwrap();
        drop(conn);
    });

    t1.join().unwrap();
    t2.join().unwrap();

    pool.get().unwrap();
}

#[test]
fn test_file() {
    let manager = SqliteConnectionManager::new("file.db").unwrap();
    let config = r2d2::Config::builder().pool_size(2).build();
    let pool = r2d2::Pool::new(config, manager).unwrap();

    let (s1, r1) = mpsc::channel();
    let (s2, r2) = mpsc::channel();

    let pool1 = pool.clone();
    let t1 = thread::spawn(move || {
        let conn = pool1.get().unwrap();
        s1.send(()).unwrap();
        r2.recv().unwrap();
        drop(conn);
    });

    let pool2 = pool.clone();
    let t2 = thread::spawn(move || {
        let conn = pool2.get().unwrap();
        s2.send(()).unwrap();
        r1.recv().unwrap();
        drop(conn);
    });

    t1.join().unwrap();
    t2.join().unwrap();

    pool.get().unwrap();
}

#[test]
fn test_is_valid() {
    let manager = SqliteConnectionManager::new(":memory:").unwrap();
    let config = r2d2::Config::builder().pool_size(1).test_on_check_out(true).build();
    let pool = r2d2::Pool::new(config, manager).unwrap();

    pool.get().unwrap();
}

#[test]
fn test_error_handling() {
    //! We specify a directory as a database. This is bound to fail.
    let dir = TempDir::new("r2d2-sqlite").expect("Could not create temporary directory");
    let dirpath = dir.path().to_str().unwrap();
    let manager = SqliteConnectionManager::new(dirpath).unwrap();
    assert!(manager.connect().is_err());
}
