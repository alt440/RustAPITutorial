# RustAPITutorial
Rust API tutorial using Warp and Tokio

# About this project
This project is a simple API that handles a counter value. The counter must be able to handle requests asynchronously, i.e., can handle racing conditions.
The API has 3 endpoints: a getter (/getCounter), an incrementer (/incrementCounter), and a decrementer (/decrementCounter). The counter starts at value 100.

The project contains two files: main and counter rs. The main file has all the handling of API requests, while counter has the handling of the counter variable.

# Other interesting information
I have been told that axum was an easier alternative to warp. Warp returns complex types, and is difficult to debug. Also, if the API has a lot of endpoint,
adding all of them to warp is barely manageable (as I know): a chaining of 'or' operations doesn't seem like the right way to do it.


Here is breakdown of axum vs warp offered by ChatGPT:

When comparing Rust web frameworks Warp and Axum, each has its strengths and weaknesses. Here's a breakdown of their advantages and disadvantages:

## Warp

Advantages:

    1. Composable Filters: Warp uses a filter system that allows for highly composable and reusable pieces of functionality. This makes it flexible for building complex applications.
    2. Performance: Warp is built on top of tokio and is known for its high performance, especially in handling asynchronous requests.
    3. Strong Type Safety: It emphasizes type safety, reducing runtime errors. The filters can enforce types at compile time.
    4. Minimal Boilerplate: Warp often requires less boilerplate code compared to some other frameworks, which can make it quicker to write straightforward applications.
    5. Streaming Support: It has excellent support for streaming data, which is useful for real-time applications.

Disadvantages:

    1. Learning Curve: The filter system, while powerful, can be complex for beginners to grasp fully.
    2. Less Opinionated: Its flexibility can lead to inconsistency in how applications are structured, especially for larger projects.
    3. Limited Built-in Middleware: While you can create custom middleware easily, it doesn’t come with as many built-in options as some other frameworks.

## Axum

Advantages:

    1. Built on Tower: Axum is built on top of the Tower ecosystem, which provides robust middleware support and other utilities for building async applications.
    2. Routing and Handler Simplicity: Axum’s routing and handler systems are intuitive and similar to Express.js, which can be easier for those coming from JavaScript backgrounds.
    3. Strong Community Support: Being part of the Tokio ecosystem, Axum benefits from extensive community support and resources.
    4. Built-in Extractors: Axum provides a variety of built-in extractors for common use cases (like JSON, form data), simplifying request handling.
    5. Good Documentation: Axum’s documentation is generally well-organized and clear, making it easier for newcomers to get started.

Disadvantages:

    1. Less Fine-Grained Composability: Compared to Warp, Axum’s composability might feel less flexible, especially for highly specialized use cases.
    2. Performance Considerations: While Axum is performant, Warp has a reputation for being slightly faster in certain benchmarks.
    3. Learning the Tower Ecosystem: Understanding the full Tower ecosystem can add complexity, particularly for developers not familiar with its concepts.

Conclusion

    - Use Warp if you need high performance, advanced filtering capabilities, and a strong emphasis on type safety.  
    - Use Axum if you prefer simplicity in routing, want a smoother learning curve, and benefit from the middleware capabilities provided by the Tower ecosystem.