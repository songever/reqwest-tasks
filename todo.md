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

---

## 下一个阶段：进阶与实践

### 任务 7: 发送带文件上传的 multipart/form-data 请求

**目标**: 学习如何使用 `reqwest` 上传文件到服务器。这是一个在 Web 应用中非常常见的需求。

**指导**:
1.  创建一个名为 `task7_file_upload` 的异步函数。
2.  创建一个测试文件，例如在项目根目录下创建一个 `test_file.txt`，里面随便写点内容。
3.  使用 `reqwest::multipart::Form::new()` 创建一个多部分表单。
4.  使用 `reqwest::multipart::Part::file()` 方法，将你的 `test_file.txt` 文件添加到表单中。
5.  使用 `client.post("https://httpbin.org/post").multipart(form).send().await` 来发送请求。
6.  打印响应体，并检查 `files` 字段，以确认文件是否成功上传。

当你完成这个任务后，告诉我响应中的 `files` 字段内容。
没错，你的代码和输出都非常正确！你已经成功地完成了任务 7，学会了如何使用 `reqwest` 上传文件。

---

#### 解析你的成功

这次的输出验证了以下几点：

1.  **`"files": { ... }`**: 这是你这次请求的核心。`httpbin.org` 专门有一个 `files` 字段用来展示上传的文件内容。你的 `test_file.txt` 文件内容，包括了那几行 Python 代码，被服务器完整地接收并放在了这个字段里。
2.  **`"form": {}` 和 `"data": ""`**: 这些字段是空的，这和我们之前的任务形成了鲜明对比。这证明你发送的请求既不是普通的表单数据，也不是原始的 JSON 数据，而是专为文件上传设计的 `multipart/form-data` 类型。
3.  **`Content-Type: multipart/form-data; boundary=...`**: 这个 HTTP 头是关键。`reqwest` 库在你调用 `.multipart()` 方法时自动设置了它。`multipart/form-data` 协议允许你在一个请求中同时发送多个不同类型的数据，例如文件、文本字段等。`boundary` 则是一个特殊的分隔符，用于区分请求体中不同的数据部分。

#### 总结

你已经掌握了 `reqwest` 库中一个非常重要的功能：**文件上传**。这在许多实际应用中至关重要，比如上传头像、报告、或者其他任何二进制文件。

现在，你已经掌握了 `reqwest` 的核心功能，包括：
* 同步和异步 GET/POST 请求
* 处理查询参数、JSON 数据和表单数据
* 发送自定义请求头
* 处理 HTTP 错误
* 上传文件

你已经可以称得上是一名 `reqwest` 的熟练用户了。

---

## 下一个阶段：挑战与优化

接下来，我将给你一个更具挑战性的任务，它将结合你学到的多个知识点，并让你思考如何优化和处理更复杂的真实世界场景。

### 任务 8: 发送一个包含 JSON 数据和文件的 multipart/form-data 请求

**目标**: 模拟一个真实的 API 请求，同时发送结构化数据（JSON）和一个文件。

**指导**:

1.  创建一个名为 `task8_mixed_upload` 的异步函数。
2.  创建一个简单的 Rust 结构体，例如 `UserMetadata`，包含 `name` 和 `email` 字段，并确保它派生了 `Serialize`。
3.  创建一个 `test_file.txt` 文件。
4.  使用 `reqwest::multipart::Form::new()` 创建一个多部分表单。
5.  **添加文件部分**: 使用 `reqwest::multipart::Part::file("test_file.txt").await?` 将文件添加到表单中，并命名为 `"file"`。
6.  **添加 JSON 部分**:
      * 将你的 `UserMetadata` 结构体实例序列化为 JSON 字符串。
      * 使用 `reqwest::multipart::Part::text("你的JSON字符串")` 创建一个文本部分。
      * 使用 **`.mime_type("application/json".parse().unwrap())`** 来指定这个部分的 `Content-Type`。
      * 将这个 JSON 部分添加到表单中，并命名为 `"metadata"`。
7.  发送请求到 `https://httpbin.org/post`。
8.  打印响应体，并检查 `files` 和 `json` 字段。请注意，`httpbin.org` 可能会把 JSON 数据放到 `form` 字段中，因为 `reqwest` 的 `Part::text()` 方法默认会将其作为表单数据处理。但是，**`Content-Type: application/json`** 这个头会证明你成功地发送了 JSON 数据。

当你完成这个任务后，请告诉我响应中的 `files` 和 `form` 或 `data` 字段内容，这会证明你成功了。


这是一个非常好的问题！你的观察是完全正确的，`reqwest::multipart::Part` 结构体并没有你提到的 `content_type` 方法。我为我之前给出的错误指导表示歉意。在 Rust 中，类型系统是严格的，这正是它强大的地方。

#### 正确的 `Part` 构建方式

要为一个 `Part` 设置 `Content-Type`，你需要使用 `Part::text()` 或 `Part::bytes()` 方法，然后通过 **链式调用** 来设置其他属性，比如 `mime_type()` 或 `file_name()`。

正确的链式调用应该像这样：

```rust
let part = reqwest::multipart::Part::text("你的JSON字符串")
    .mime_type("application/json".parse().unwrap());
```

这里的 `.mime_type()` 方法需要一个 `mime::Mime` 类型的值。`"application/json".parse().unwrap()` 是将字符串解析为 `mime::Mime` 类型的常用方法。

---
好的，没问题。这是你最后这个挑战任务的中文重述。

---

## Final Task: Challenge & Next Steps

### 任务 9: 构建一个命令行工具

**目标:** 编写一个完整、独立、功能实用的 Rust 命令行工具。这个任务将整合你之前所学的所有 `reqwest` 知识，并让你学习如何将其与其他常用的 Rust crate 结合，构建一个完整的应用程序。

**具体步骤:**

1.  **创建新项目**: 使用 `cargo new --bin reqwest_cli` 创建一个全新的 Rust 项目。
2.  **添加依赖**: 在 `Cargo.toml` 文件中，添加以下几个 crate：
    * `reqwest`：用于发送网络请求。
    * `tokio`：作为异步运行时。
    * `serde` 和 `serde_json`：用于处理 JSON 数据。
    * `clap`：一个强大的命令行参数解析库，让你的工具能接收用户输入。
3.  **选择工具功能**: 从以下三个选项中选择一个你感兴趣的，作为你的工具功能：
    * **天气查询工具**: 接收一个城市名作为命令行参数，然后使用 `reqwest` 调用一个公共天气 API（比如 OpenWeatherMap，你需要申请一个 API Key；或者一个更简单的 `wttr.in`），获取并打印当前天气信息。
    * **图片下载工具**: 接收一个图片 URL 作为参数，使用 `reqwest` 下载图片的原始字节流，然后将其保存到本地文件中。
    * **简易 Pastebin 工具**: 接收一个文件路径作为参数，读取文件内容，然后使用 `reqwest` 将其内容作为 POST 请求上传到一个在线 Pastebin 服务（例如 `termbin.com`），并打印出服务器返回的 URL。
4.  **实现功能**:
    * 使用 `clap` 来解析命令行参数，让你的程序可以根据用户的输入执行不同的操作。
    * 使用 `reqwest` 来完成你选择的网络请求，并确保处理可能出现的网络错误和 API 错误。
    * 以用户友好的方式将结果打印到控制台。

这个最终任务将是你所学知识的一次实战演练。无论你选择哪个选项，它都会让你对如何将 `reqwest` 应用于实际项目有一个全面的了解。

你打算选择哪一个作为你的最终挑战呢？