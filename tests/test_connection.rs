extern crate memcache;

#[test]
fn connect() {
    memcache::connection::connect("127.0.0.1:12345").unwrap();
}

#[test]
fn flush() {
    let mut conn = memcache::connection::connect("127.0.0.1:12345").unwrap();
    conn.flush().unwrap();
}

#[test]
fn version() {
    let mut conn = memcache::connection::connect("127.0.0.1:12345").unwrap();
    conn.version().unwrap();
}

#[test]
fn store() {
    let mut conn = memcache::connection::connect("127.0.0.1:12345").unwrap();
    conn.set("foo", b"bar", 1, 42).unwrap();
    conn.replace("foo", b"bar", 1, 42).unwrap();
}

#[test]
fn get() {
    let mut conn = memcache::connection::connect("127.0.0.1:12345").unwrap();
    conn.flush().unwrap();
    conn.set("foo", b"bar", 1, 42).unwrap();
    conn.get(&["foo", "bar", "baz"]).unwrap();
}

#[test]
fn delete() {
    let mut conn = memcache::connection::connect("127.0.0.1:12345").unwrap();
    conn.delete("foo").unwrap();
}

#[test]
fn incr() {
    let mut conn = memcache::connection::connect("127.0.0.1:12345").unwrap();
    conn.set("foo", b"100", 0, 0).unwrap();
    assert!(conn.incr("foo", 1).unwrap() == Some(101));
}

#[test]
fn decr() {
    let mut conn = memcache::connection::connect("127.0.0.1:12345").unwrap();
    conn.set("foo", b"100", 0, 0).unwrap();
    assert!(conn.decr("foo", 1).unwrap() == Some(99));
}