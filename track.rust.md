## 则

|                |              | 限制                     |                                |
| -------------- | ------------ | ------------------------ | ------------------------------ |
| package name   | 包名         | 不能数字开头，不能有空格 |                                |
| directory name | 包的文件夹名 |                          |                                |
| binary name    | 二进制名     |                          | 可以在 Cargo.toml 单独制定名字 |
| library name   | 库名         | 不能使用中划线 `-`       | 可以在 Cargo.toml 单独制定名字 |

- 默认情况：
  1. 包名取自包的文件夹名
  2. 二进制名取自包名
  3. 库名取自包名

库名：在默认的库名取自包名情况下，如果在包名含有中划线的情况，库名将会默认转化为下划线 `_`

包名：characters must be Unicode XID characters (numbers, `-`, `_`, or most letters)
