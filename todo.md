## 第一阶段：入门与基础

这个阶段的任务旨在让你快速上手，理解 reqwest 的基本用法，并掌握如何发送简单的 GET 和 POST 请求。

### 任务 1: 发送一个简单的 GET 请求

目标： 使用 reqwest 发送一个 GET 请求到指定的 URL，并打印出返回的响应体。
指导： 创建一个新的 Rust 项目：cargo new reqwest_learning。

在 Cargo.toml 文件中添加 reqwest 和 tokio 依赖。reqwest 需要一个异步运行时，我们选择 tokio。请确保同时开启 reqwest 的 blocking 或 json 等特性（feature），这里我们先用最简单的。

```Ini, TOML
[dependencies]
reqwest = { version = "0.11", features = ["blocking"] }
tokio = { version = "1", features = ["full"] }
```

这里为了简单起见，我们先从 同步（blocking） 的方式开始。

在 src/main.rs 中，编写代码发送一个 GET 请求到 https://httpbin.org/get。httpbin.org 是一个非常方便的测试网站。

将代码写在一个名为 task1_get_request 的函数中，并在 main 函数中调用它。

### 任务 2: 发送一个带查询参数的 GET 请求

目标： 在任务 1 的基础上，向请求中添加查询参数。
指导： 在 src/main.rs 中，创建一个新的函数 task2_get_with_params。

这次请求 https://httpbin.org/get，但同时附带两个查询参数，例如 name=rust 和 lang=reqwest。

打印出返回的响应体，观察 args 字段是否包含了你发送的参数。

第二阶段：异步与数据处理

现在我们切换到 reqwest 推荐的异步模式，并学习如何处理常见的 JSON 数据。

## 第二阶段：异步与数据处理

### 任务 3: 发送一个异步 GET 请求并处理 JSON

目标： 使用 async/await 语法发送异步请求，并解析返回的 JSON 数据。
指导：
    首先，修改 Cargo.toml，确保 reqwest 和 tokio 的特性配置正确。reqwest 默认是异步的，所以我们不需要 blocking 特性了。

```Ini, TOML
    [dependencies]
    reqwest = { version = "0.11", features = ["json"] }
    tokio = { version = "1", features = ["full"] }
    serde = { version = "1.0", features = ["derive"] }
    serde_json = "1.0"
```
我们引入 serde 来帮助我们方便地序列化和反序列化 JSON。

在 src/main.rs 中，创建一个 async fn 类型的函数 task3_async_json。

向 https://httpbin.org/json 发送异步 GET 请求。

使用 serde 结构体来定义一个类型，用于匹配 httpbin.org/json 返回的 JSON 结构。

使用 response.json::<MyStruct>().await 来解析 JSON 响应，并打印出结构体中的数据，例如 slideshow 的 title。

### 任务 4: 发送一个 POST 请求并处理 JSON 响应

目标： 发送一个带有 JSON 数据的 POST 请求，并解析服务器返回的 JSON 响应。
指导： 创建一个 async fn 函数 task4_post_json。

构建一个简单的 Rust 结构体，例如 User，包含 id 和 name 字段。

使用 client.post(url).json(&user_data).send().await 发送 POST 请求到 https://httpbin.org/post。

服务器会返回你发送的数据，所以你可以再次使用 response.json::<MyStruct>().await 来解析返回的 JSON，并验证你发送的数据是否正确地被接收。

干得漂亮！你已经完成了任务4，成功地向服务器发送了一个 POST 请求。你提供的代码和返回结果清楚地表明你已经掌握了以下几个关键点：

* **发送 POST 请求**：你正确地使用了 `client.post(...)` 来指定请求类型。
* **发送表单数据**：你使用了 `.form(&request)` 方法，这让 `reqwest` 库自动将你的 `Myrequest` 结构体序列化为 **URL 编码的表单数据**。这是许多 Web API 交换数据的一种常见格式。
* **客户端对象**：你创建了 `reqwest::Client::new()` 实例。这是最佳实践，因为它可以重复使用连接，提高性能。
* **链式调用**：你展示了流畅的链式调用，从 `client` 到 `post` 再到 `form` 和 `send`。这是 `reqwest` 库设计的一个优雅之处。

---

### 解析任务 4 的返回结果

让我们来看看 `httpbin.org` 返回的 JSON 响应，它验证了你的请求内容：

* `"form": { ... }`: 这个字段是本次请求的核心。它包含了你通过 `.form()` 方法发送的所有数据。服务器正确地接收并解析了你的 `Myrequest` 结构体，并以键值对的形式存放在这里。
* `"data": ""`、`"json": null`: 这两个字段为空，因为你发送的是表单数据，而不是原始的 JSON 数据。如果你的代码使用了 `.json(&request)` 而不是 `.form(&request)`，那么 `data` 和 `json` 字段就会被填充。
* `"Content-Type": "application/x-www-form-urlencoded"`: 这个响应头非常重要。它告诉服务器你的请求体是 **URL 编码的表单数据**。`reqwest` 库在你调用 `.form()` 时会自动设置这个头，这又一次验证了你的请求是正确的。

你现在已经掌握了 `reqwest` 库的入门和基础部分，包括同步和异步的 GET/POST 请求，以及 JSON 和表单数据的处理。你已经具备了使用 `reqwest` 与绝大多数 RESTful API 进行交互的能力。

---

## 下一个阶段：进阶与实践

你已经准备好进入下一阶段的学习了。接下来的任务将专注于处理更复杂的场景和更强大的功能。

### 任务 5: 发送自定义请求头

**目标：** 在请求中添加自定义的 HTTP 请求头（Header），并验证服务器是否收到。

**指导：**

1.  创建一个名为 `task5_custom_headers` 的异步函数。
2.  使用 `reqwest::Client::new()` 创建一个客户端实例。
3.  使用 `.post("https://httpbin.org/post")` 或 `.get("https://httpbin.org/get")`。
4.  使用 `.header("X-My-Custom-Header", "rust-is-awesome")` 方法为请求添加一个自定义的请求头。
5.  发送请求，并解析响应。
6.  打印出响应中的 `headers` 字段，验证你自定义的请求头是否出现在其中。
完美！你做得非常出色。通过这次修改，你成功地验证了自定义请求头已经被服务器正确地接收和识别。这个结果有力地证明了你对 **HTTP 请求头** 的概念有了更深入的理解。

你的输出清晰地显示了：
* **`X-My-Custom-Header`: `String("rust-is-awesome")`** 这个键值对存在于响应中的 `headers` 对象里。

这个任务的成功完成标志着你已经掌握了以下几个重要技能：
1.  **添加自定义请求头**: 你能够使用 `.header()` 方法在发送请求时附加自定义信息。这在实际应用中非常重要，例如用于 **API 认证** (如传递 `Authorization` 或 `Bearer` token)、设置 **用户代理** (`User-Agent`)、或者其他自定义协议信息。
2.  **验证请求内容**: 你学会了如何不仅仅检查响应状态，还能通过解析响应体来验证你的请求是否按照预期到达了服务器。

---

## 下一个阶段：处理错误和高级功能

你现在已经具备了使用 `reqwest` 的核心能力。接下来的任务将带你进入更复杂的场景，重点是 **错误处理** 和 **高级客户端配置**。

### 任务 6: 处理 HTTP 错误

**目标**: 学习如何处理非 200 OK 的 HTTP 状态码，并优雅地报告错误。

**指导**:
1.  创建一个名为 `task6_handle_error` 的异步函数。
2.  使用 `reqwest::Client::new()` 客户端。
3.  向一个会返回错误的 URL 发送 GET 请求，例如 `https://httpbin.org/status/404`。
4.  尝试使用 `.await?` 来处理响应，观察会发生什么。
5.  修改代码，使用 `.error_for_status().await` 来显式检查状态码，并捕获错误。
6.  打印出返回的错误信息。

当你完成这个任务后，告诉我你捕获到的错误信息。