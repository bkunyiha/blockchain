# Traits: Polymorphism and Code Reuse

With structs and enums, we can model our domain concepts. But as our blockchain implementation grows, we find ourselves needing to express that different types share common behaviors. A transaction can be validated, a block can be hashed, and a wallet address can be encoded. Rust's trait system allows us to define these shared behaviors and implement them for different types, providing polymorphism and code reuse without the overhead of traditional object-oriented inheritance.

Traits are Rust's mechanism for defining shared behavior. They're similar to interfaces in other languages but more powerful, enabling both static and dynamic dispatch, associated types, and default implementations. In this chapter, we'll explore how traits enable flexible, reusable code in our blockchain.

### Defining Traits: Specifying Behavior

When we define a trait, we're specifying a contract that types must fulfill. Consider what behaviors our blockchain types might share:

```rust
// Conceptual example - traits define interfaces
trait Hashable {
    fn hash(&self) -> Vec<u8>;
}

trait Validatable {
    fn validate(&self) -> Result<()>;
}
```

These trait definitions specify that any type implementing `Hashable` must provide a `hash()` method, and any type implementing `Validatable` must provide a `validate()` method. The trait doesn't specify how these methods work—that's left to each implementing type—but it guarantees that the methods exist.

### Implementing Traits: Providing Behavior

Once we've defined a trait, types can implement it by providing the required methods. Implementation happens in `impl` blocks, which can be associated with specific types or made generic over types that implement certain traits.

**Example from `bitcoin/src/node/peers.rs`:**

```rust
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Node {
    addr: SocketAddr,
}

impl Node {
    fn new(addr: SocketAddr) -> Node {
        Node { addr }
    }

    pub fn get_addr(&self) -> SocketAddr {
        self.addr
    }
}
```

This implementation block shows how we define behavior for the `Node` struct. The `impl Node` block contains methods that operate on `Node` instances. Notice the different `self` types:

- **`new(addr: SocketAddr)`**: An associated function (no `self` parameter) that creates a new `Node`. Called with `Node::new(addr)`.
- **`get_addr(&self)`**: A method that takes an immutable borrow of `self`. Called with `node.get_addr()` and allows reading but not modifying.
- **`&mut self`**: Would allow modifying the struct (not shown here but used elsewhere).
- **`self`**: Would take ownership of the struct (rare, used for consuming operations).

These different `self` types give us fine-grained control over how methods interact with their data, enabling both efficient access patterns and clear ownership semantics.

### Implementing Standard Traits: Default

Rust's standard library provides many useful traits that types can implement. One of the most common is `Default`, which provides a way to create default values for types. In our node management code, we implement `Default` for the `Nodes` collection:

```rust
impl Default for Nodes {
    fn default() -> Self {
        Self::new()
    }
}
```

By implementing `Default`, we enable several convenient patterns. Code can call `Nodes::default()` or use `Default::default()` to create a new `Nodes` instance. Some Rust features, like struct field initialization, can automatically use `Default` implementations. The trait can also be derived automatically with `#[derive(Default)]` when all fields implement `Default`, but in our case, we provide a custom implementation that calls our `new()` method.

This pattern of implementing standard library traits appears throughout our codebase, enabling our types to work seamlessly with Rust's ecosystem and providing familiar interfaces for common operations.

### Trait Bounds: Constraining Generic Types

When we write generic code, we often need to specify what capabilities the generic types must have. Trait bounds allow us to constrain generic type parameters, ensuring they implement the traits we need. This enables generic code that's both flexible and type-safe.

**Example from `bitcoin/src/chain/chainstate.rs`:**

```rust
async fn read<F, Fut, T>(&self, f: F) -> Result<T>
where
    F: FnOnce(BlockchainFileSystem) -> Fut + Send,  // ← Trait bounds
    Fut: Future<Output = Result<T>> + Send,
    T: Send + 'static,
{
    let blockchain_guard = self.0.read().await;
    f(blockchain_guard.clone()).await
}
```

**Trait Bounds:**
- **`F: FnOnce(...)`**: Function that can be called once
- **`+ Send`**: Can be safely sent between threads
- **`+ 'static`**: Lives for the entire program duration

## Summary

Traits enable polymorphism and code reuse in Rust without the overhead of traditional object-oriented inheritance. They allow us to define shared behavior that types can implement, enabling generic code that works with any type implementing a trait.

Through trait bounds, we can constrain generic types to ensure they have the capabilities we need. This enables flexible, reusable code while maintaining type safety. Traits are central to Rust's design, appearing throughout the standard library and our blockchain implementation.

In the next chapter, we'll explore how Rust handles errors explicitly through the `Result` and `Option` types, building on the trait concepts we've learned here.

---

## Navigation

- **[← Previous: Data Structures](03-Data-Structures.md)** - Structs and Enums
- **[Next: Error Handling →](05-Error-Handling.md)** - Result and Option types
- **[Rust Guide Index](README.md)** - Complete guide overview and table of contents
- **[Generics](06-Generics.md)** - Generic code with trait bounds
- **[Derive Macros](10-Derive-Macros.md)** - Automatic trait implementations
- **[Modules](13-Modules.md)** - Code organization

**Related Guides:**
- **[Transaction ID Format](../bitcoin-blockchain/primitives/02-Transaction-ID-Format.md)** - See traits in action
- **[Web API Architecture](../bitcoin-blockchain/web/README.md)** - Trait-based design patterns

---

<div align="center">

**📚 [← Rust Guide Index](README.md)** | **Traits** | **[← Previous: Data Structures](03-Data-Structures.md)** | **[Next: Error Handling →](05-Error-Handling.md)** 📚

</div>

---

*This chapter covers traits and polymorphism. Continue to [Error Handling](05-Error-Handling.md) to learn how Rust handles errors explicitly.*


*For more Rust language features, see the [Rust Guide Index](README.md).*