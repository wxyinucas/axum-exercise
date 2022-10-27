# 介绍

用于Axum crate的练习，
基于[Axum中文网](https://axum.rs/subject/roaming-axum)。

用splx代替了原有postgresql的链接方案，这个crate更灵活，能独立的完成之前多个crates的组合功能。

## 需要进一步学习的部分

### Html url 和 重定向

#### roaming

- upload: 如何用 html 激活 `post()`?
- session: form、html 和 `post()`?
    - form 中信息怎么传递？html中，axum中？

#### blog
- `handler`: 重定向。

### Crate 使用方法

#### roaming

- upload: 如何上传文件？ `axum: Multipart, ContentLengthLimit`
- session: `HeaderMap` & `Cookie`代表的是？常用的方法有？
    - `header` 与 重定向？
- template: 了解 `askama::Template.`
    - 母模版 与 子模板。

#### blog
- `db/implement`: 怎么处理`COUNT(*)`？
- `cookie`: `Option`中`and_then()`和`map()`的区别是什么？
- `model`: `time::SystemTime` 怎么和 `String` 格式变化？如何对时间格式格式化`chrono`？
- `password`: crate `bcrypt`，解决了什么需求，怎么解决的，怎么交互？

### 背景知识

#### roaming

- session: `cookie`与`session`的关系？

### 架构设计

#### todo_system

- server: 总结设计要点--router，page，model，view，handler，db；如何交互？

#### blog
- `backend/handlers/auth`: log，在`error`中用`event`触发，用`span`记录位置信息。
- `backend/handlers/category`: `redirect()`的设计与使用，用哪了，和谁交互了？
- `backend/handlers/topic`: 如何设计暴露的接口，
- `backend/mod`: 如何设计router，以及各个 mod 暴露的接口。
- `db/structs`: 分页，为了解决什么问题，需要有什么功能，怎么和其他部分交互？
- `db/traits`: 写代码流程，先model，再form再trait。
- `cookie`: cookie怎么和session交互，cookie有啥用？
- `middleware`: 这个中间件有什么用，怎么使用的，如何交互？


### 测试驱动

#### todo_system

- db/mod: 如何设计trait的测试？

#### blog
- `db/implement`: 如何对trait进行测试，如何查看test时的输出？


## 总结
- 处理重定向，与form 与 router。（什么时候用，在哪用，怎么用）
  - 妈的，渲染模版才更没懂好吧！`askama::Template`->`Jinja like tmplate rander`
- server: 总结设计要点--router，page，model，view，handler，form，db；如何交互？
  - 大致可分为：提交任务的界面，展示结果的界面，和 接受任务、实际用于处理、并重定向到展示结果页面的handler。
  - model，就是rust内的数据结构；view和用于展示的page 一一对应；handler作用如上，分为3类。
  - 注意：router中复杂数据结构的输入，一定是在form中，所以输入之前必然有一个用于展示的page。
- 如何设计mod与暴露接口。
  - 在设计与implement时分析，哪一部分会用到现在写的代码。
  - 一个模块里的代码默认对其父模块私有。
  - `pub use`可以使得外部使用时，结构与真实结构不同。
- 如何设计更好用的测试，如何现在测试内容。
  - 敲定trait就写test。
  - `cargo test -- --show-output`。
- 开发span logger。(等水平再高点，写一个带数据采集的版本。)
- 如何处理headers & cookies。
  - header存在与`请求`和`响应`中。
  - cookie存在于请求字段。
  - set-cookie存在于响应字段。