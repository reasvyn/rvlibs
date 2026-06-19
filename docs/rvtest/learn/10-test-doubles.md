# Chapter 10 — Test Doubles

[← Previous](09-setup-and-teardown.md) • [Index](00-index.md) • [Next →](11-mocking-external-deps.md)

---

When testing a unit of code, you often need to isolate it from its
dependencies.  A function that calls an HTTP API or reads from a database is
no longer a unit test — it is an integration test.  Test doubles are
replacements for real dependencies that give you control and observability.

---

## The Terminology

The literature uses several terms for test doubles.  They are often confused,
but they mean different things:

| Term | What It Does |
|------|-------------|
| **Dummy** | Passed around but never used (filling a parameter) |
| **Fake** | A working implementation that is unsuitable for production (in-memory database) |
| **Stub** | Returns fixed values, no logic |
| **Spy** | Records how it was called, can assert on call history |
| **Mock** | Pre-programmed expectations (call this method N times with these arguments) |

---

## When to Use Each

### Stub — Return Fixed Values

Use when you need a dependency to return a specific value:

```rust
struct ConfigStub;
impl ConfigLoader for ConfigStub {
    fn load(&self) -> Config {
        Config { host: "localhost".into(), port: 8080 }
    }
}

fn start_server(config: &dyn ConfigLoader) {
    let cfg = config.load();
    // ...
}

#[test]
 fn server_starts_with_stub_config() {
     let stub = ConfigStub;
     start_server(&stub); // Uses stub config, not real file I/O
 }
```

### Spy — Record and Assert on Calls

Use when you need to verify that a function was called with specific
arguments:

```rust
struct EmailSpy {
    sent_messages: std::sync::Mutex<Vec<String>>,
}

impl EmailSpy {
    fn new() -> Self {
        EmailSpy { sent_messages: std::sync::Mutex::new(Vec::new()) }
    }
}

impl EmailSender for EmailSpy {
    fn send(&self, to: &str, body: &str) {
        self.sent_messages.lock().unwrap()
            .push(format!("{to}: {body}"));
    }
}

#[test]
 fn welcome_email_is_sent() {
     let spy = EmailSpy::new();
     register_user("alice@example.com", &spy);
     let sent = spy.sent_messages.lock().unwrap();
     assert_eq!(sent.len(), 1);
     assert!(sent[0].contains("alice@example.com"));
     assert!(sent[0].contains("Welcome"));
 }
```

### Fake — Working but Simplified Implementation

Use when the real dependency is too slow, unreliable, or hard to set up:

```rust
// Real implementation connects to PostgreSQL
struct PostgresDb;
impl Database for PostgresDb {
    fn query_user(&self, id: u32) -> Option<User> {
        // Real SQL query
        todo!()
    }
}

// Fake uses an in-memory HashMap
struct InMemoryDb {
    users: std::sync::Mutex<std::collections::HashMap<u32, User>>,
}

impl Database for InMemoryDb {
    fn query_user(&self, id: u32) -> Option<User> {
        self.users.lock().unwrap().get(&id).cloned()
    }
}

#[test]
 fn test_with_fake_database() {
     let db = InMemoryDb { users: std::sync::Mutex::new(std::collections::HashMap::new()) };
     db.users.lock().unwrap().insert(1, User { id: 1, name: "Alice".into() });
     assert_eq!(db.query_user(1).unwrap().name, "Alice");
 }
```

---

## Designing for Testability

The key requirement for test doubles is that the code being tested must accept
dependencies through a **trait** or **function pointer**:

```rust
// ❌ Hard to test — directly calls a concrete implementation
fn process_order(order: Order) {
    let db = PostgresDb::new();
    db.save_order(&order);
    EmailSender::new().send_confirmation(&order);
}
```

```rust
// ✅ Easy to test — accepts trait implementations
fn process_order(order: Order, db: &impl Database, email: &impl EmailSender) {
    db.save_order(&order);
    email.send_confirmation(&order);
}
```

This is called **dependency injection**.  It is the single most impactful
change you can make to improve testability.

---

## Traits vs Function Pointers

For simple cases, a function pointer is lighter than a trait:

```rust
// Trait approach
trait IdGenerator {
    fn generate(&self) -> u64;
}

// Function pointer approach
struct UserService {
    generate_id: fn() -> u64,
}

#[test]
 fn user_service_with_stub_id_generator() {
     let service = UserService {
         generate_id: || 42, // Stub: always returns 42
     };
     let user = service.create_user("Alice");
     assert_eq!(user.id, 42);
 }
```

---

## When You Cannot Change the Code

Sometimes you are testing legacy code where you cannot modify the function
signature.  In that case, `rvtest` provides `patch!` to temporarily replace a
function (covered in detail in the next chapter):

```rust
// real implementation
fn get_current_time() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

#[test]
 fn test_with_frozen_time() {
     // Temporarily replace the function
     let _guard = rvtest::mock::patch!("my_crate::get_current_time", || 1000);
     let result = calculate_expiry();
     assert_eq!(result, 1000 + 3600); // Uses frozen time
 }
```

---

## A Practical Example

Here is a complete example showing all four types of test doubles:

```rust
use std::sync::Mutex;

// --- Traits ---
trait Database {
    fn save_user(&self, user: &User);
    fn get_user(&self, id: u32) -> Option<User>;
}

trait EmailSender {
    fn send(&self, to: &str, subject: &str);
}

// --- Production code ---
fn register_user(
    name: &str,
    email: &str,
    db: &impl Database,
    mailer: &impl EmailSender,
) -> User {
    let user = User { id: 1, name: name.into(), email: email.into() };
    db.save_user(&user);
    mailer.send(email, "Welcome!");
    user
}

// --- Test doubles ---
struct DbFake {
    users: Mutex<std::collections::HashMap<u32, User>>,
}

impl Database for DbFake {
    fn save_user(&self, user: &User) {
        self.users.lock().unwrap().insert(user.id, user.clone());
    }
    fn get_user(&self, id: u32) -> Option<User> {
        self.users.lock().unwrap().get(&id).cloned()
    }
}

struct MailerSpy {
    messages: Mutex<Vec<String>>,
}

impl EmailSender for MailerSpy {
    fn send(&self, to: &str, subject: &str) {
        self.messages.lock().unwrap().push(format!("{to}: {subject}"));
    }
}

// --- Tests ---
#[test]
 fn register_user_saves_to_database() {
     let db = DbFake { users: Mutex::new(std::collections::HashMap::new()) };
     let mailer = MailerSpy { messages: Mutex::new(Vec::new()) };

     let user = register_user("Alice", "alice@example.com", &db, &mailer);

     // Verify database state
     let saved = db.get_user(1);
     assert_eq!(saved, Some(user));

     // Verify email was sent
     let msgs = mailer.messages.lock().unwrap();
     assert_eq!(msgs.len(), 1);
     assert!(msgs[0].contains("alice@example.com"));
     assert!(msgs[0].contains("Welcome"));
 }
```

---

## Common Mistakes

### Using Mocks for Everything

Stubs and fakes are simpler and more maintainable than mocks.  Use a mock only
when you need to verify specific call patterns (how many times, with which
arguments, in which order).  For most cases, a stub or fake is sufficient.

### Over-Mocking

```rust
// ❌ Every single dependency is mocked, even trivial ones
#[test]
 fn test_with_8_mocks() {
     let mock_a = MockA::new();
     let mock_b = MockB::new();
     // ... 6 more mocks
 }
```

If a test needs more than 2-3 test doubles, consider whether the code under
test is doing too much.

### Testing Implementation Details

```rust
// ❌ Tests that the method was called, not the behaviour
#[test]
 fn test_save_user_calls_database() {
     let spy = DatabaseSpy::new();
     save_user(&spy, User::new("Alice"));
     assert_eq!(spy.save_call_count(), 1); // Implementation detail
 }
```

Instead, verify the observable behaviour (the user is saved and retrievable).

---

## Summary

- **Test doubles** replace real dependencies to isolate the code under test
- **Stubs** return fixed values; **Spies** record calls; **Fakes** are working
  simplified implementations
- Design for testability by accepting dependencies through traits
- Use dependency injection to make code testable
- Prefer stubs and fakes over mocks for most scenarios
- The next chapter covers `rvtest`'s built-in `Spy` and `patch!` tools

---

[← Previous](09-setup-and-teardown.md) • [Index](00-index.md) • [Next →](11-mocking-external-deps.md)
