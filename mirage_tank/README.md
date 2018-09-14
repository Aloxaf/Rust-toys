幻影坦克快速发车工具
--

## 原理及介绍

[幻影坦克架构指南(一)](https://zhuanlan.zhihu.com/p/31164700)

[幻影坦克架构指南(三)](https://zhuanlan.zhihu.com/p/32532733)

[棋盘格与幻影坦克](https://zhuanlan.zhihu.com/p/33148445)

## 用法
```text
mirage_tank 1.0
Aloxaf <aloxafx@gmail.com>
幻影坦克快速发车工具

USAGE:
    mtank [FLAGS] [OPTIONS] <wimage> <bimage>

FLAGS:
    -c, --colorful    发彩色车 (默认黑白)
    -f, --force       不询问直接覆盖文件
    -h, --help        Prints help information
    -s, --sparse      启用棋盘格化渲染
    -V, --version     Prints version information

OPTIONS:
        --bcolor <bcolor>    黑底图像色彩保留比例 (默认 0.7)
        --blight <blight>    黑底图像亮度 (默认 0.2)
        --bscale <bscale>    黑底图像缩放比例 (默认 1.0)
    -o, --output <output>    输出文件, png 格式 (默认 output.png)
        --wcolor <wcolor>    白底图像色彩保留比例 (默认 0.5)
        --wlight <wlight>    白底图像亮度 (默认 1.0)
        --wscale <wscale>    白底图像缩放比例 (默认 1.0)

ARGS:
    <wimage>    白底下显示的图片(表层)
    <bimage>    黑底下显示的图片(里层)
```
