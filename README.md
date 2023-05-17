# 一个IO项目：构建一个命令行程序

`grep` 项目将会结合之前所学的一些内容：

* 代码组织（使用 [第七章](https://kaisery.github.io/trpl-zh-cn/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html) 学习的模块）
* vector 和字符串（[第八章](https://kaisery.github.io/trpl-zh-cn/ch08-00-common-collections.html)，集合）
* 错误处理（[第九章](https://kaisery.github.io/trpl-zh-cn/ch09-00-error-handling.html)）
* 合理的使用 trait 和生命周期（[第十章](https://kaisery.github.io/trpl-zh-cn/ch10-00-generics.html)）
* 测试（[第十一章](https://kaisery.github.io/trpl-zh-cn/ch11-00-testing.html)）

`grep`。grep 是 “**G**lobally search a **R**egular **E**xpression and **P**rint.” 的首字母缩写。`grep` 最简单的使用场景是在特定文件中搜索指定字符串。为此，`grep` 获取一个文件路径和一个字符串作为参数，接着读取文件并找到其中包含字符串参数的行，然后打印出这些行。

2023-05-17
这算是我个人学习Rust语法细则的中期产出吧，上传至此。