# BNF

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
