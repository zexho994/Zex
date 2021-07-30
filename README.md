# Zex
不借助任何工具，纯手写，编译器。

- 基于DFS的Token lexer
- 使用递归下降构建AST，右递归解决循环递归。
- 基于栈作用域+符号表进行语义分析
- 目标格式为JVM字节码格式（ing...)

## 运行

```text

 {
	int a = 5 + 5;
	echo a;
 }

 { 
	int a = 20; 
	echo a;
 }

 int a = 30;
 echo a;

 a = 40;
 echo a;

 a = 30 + 20;
 echo a;

 int b = 2 + 2 * 3;
 echo b;

 b = 2 * 10 + 2 * 10;
 echo b;

 fn foo () {
	 int c = 30;
	 echo c;
 }

```
![](https://tva1.sinaimg.cn/large/008i3skNgy1gsyt65qj8dj301704rt8i.jpg)


## BNFS

```text

<program> ::= <statements>

// 语句集：块语句，普通语句，
<statements> ::= <blockStm> | <statement> | <statements> <statement>

// 块语句
<blockStm> ::= { <statements> }

// 语句类型：分配声明语句，表达式语句，赋值语句
<statement> ::= <echo> | <declare> | <expressionStm> | <assignmentStm>

// echo a;
<echo> ::= echo (<intLiteral> | <id> | <expressionStm>) ;

// 声明语句现在提供变量声明，以后还有方法声明、类声明
<declare> ::= <classDeclare> | <varDeclare> | <fnDeclare>

// 类声明
<classDeclare> ::= class <id> <blockStm>

// 方法声明
// fn foo () {}
// fn foo () -> {}
<fnDeclare> ::= fn <id> <params> <returnType>? <blockStm>

<params> ::= ( <param>* )

<param> ::= <varDefine>

<returnType> ::= <Arrow> <type>

// 箭头指向符
<Arrow> ::= ->

// 变量声明有两种，有无初始化
<varDeclare> ::= (<varDefine> | <varDefine> <assignment> <expressionStm>) ;

// 变量定义
<varDefine> ::= <type> <id>

//变量类型，暂时提供 int
<type> ::= int

// 表达式语句
<expressionStm> ::= <addExpr>

// 赋值语句
<assignmentStm> ::= <identifier> <assignment> <expressionStm> ;

// 赋值符号
<assignment> ::= =

<addExpr> ::= <mulExpr> | <mulExpr> '+' <addExpr>

<mulExpr> ::= <primary> | <primary> '\*' <mulExpr>

<primary> ::= <id> | <intLiteral>

<id> ::= ([a-z][a-z])\*

<intLiteral> ::= [1-9][0-9]\*

<echo> ::= echo (<id> |<intLiteral>)
```

## 代码风格

```test
class Sample {
	static foo(str[] args) {
		echo("hello word);
	}
}
```

### 类声明

```json
// pub == public
pub class A {
}

// pri == private
pri class B {
}

// pro == protected
pro class C {
}
```

### 方法声明

- fn id (){}

```
fn A (int b) -> C {

} 

```

### 变量声明

```json
var a = 1
var b = "1"
var c = true
var d = 1.12
```

## 作用域
![](https://s3.us-west-2.amazonaws.com/secure.notion-static.com/80f0aabc-b794-4c36-a9fa-6503cb5f9b61/Untitled.png?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=AKIAT73L2G45O3KS52Y5%2F20210730%2Fus-west-2%2Fs3%2Faws4_request&X-Amz-Date=20210730T031140Z&X-Amz-Expires=86400&X-Amz-Signature=33f939fa6b05591c9521919db59ac764346d70c1c866f90758b558afe1edf9a4&X-Amz-SignedHeaders=host&response-content-disposition=filename%20%3D%22Untitled.png%22)
