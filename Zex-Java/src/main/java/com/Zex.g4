//grammar是规则文件的头，需要与文件名保持一致。当antlr生成词法语法解析的规则代码时，类名就是根据grammar的名字来的
grammar Zex;

//RULES: rule是antlr生成词法语法解析的基础。包括了lexer与parser，每条规则都是key:value的形式，以分号结尾。lexer首字母大写，lexer小写
compilationUnit : ( variable | print )* EOF; //root rule - our code consist consist only of variables and prints (see definition below)
variable : VARIABLE ID EQUALS value; //requires VAR token followed by ID token followed by EQUALS TOKEN ...
print : PRINT ID ; //print statement must consist of 'print' keyword and ID
value : op=NUMBER
      | op=STRING ; //must be NUMBER or STRING value (defined below)

//TOKENS
VARIABLE : 'var' ; //VARIABLE TOKEN must match exactly 'var'
PRINT : 'print' ;
EQUALS : '=' ; //must be '='
NUMBER : [0-9]+ ; //must consist only of digits
STRING : '"'.*'"' ; //must be anything in qutoes
ID : [a-zA-Z0-9]+ ; //must be any alphanumeric value
WS: [ \t\n\r]+ -> skip ; //special TOKEN for skipping whitespaces