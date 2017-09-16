# blog-cli
> A geek cli blog

## 需求 🔧
1. node.js
2. vmd
3. emacs
4. no more

## 命令 👿

1. 创建分类

```shell
blog create-type --title "这是标题啦🎫"  --sort 1
```

2. 创建文章

```shell
blog create-post --title "这是文章标题哦❤️"

## 这个命令会自动打开emacs并编辑内容，当退出emacs时会自动提交到服务器
```

3. 编辑文章

```shell
blog edit "这是文章标题哦❤️" 

## 这个命令会从服务器读取标题为“这是文章标题哦❤️”的文章并自动打开emacs并编内容
## 当退出emacs时会自动提交到服务器
```

4. 查看文章

```shell
blog show "这是文章标题哦❤️"

## 这个命令会从服务器读取标题为“这是文章标题哦❤️”的文章并自动显示
```

5. 删除文章 

```shell
blog delete "这是文章标题哦❤️"

## 这个命令会从服务器删除标题为“这是文章标题哦❤️”的文示
```

