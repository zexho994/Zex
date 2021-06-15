package parse;


import ast.*;
import exception.ParseException;
import lexer.Lexer;
import token.Token;

import java.util.HashSet;

/**
 * @author Zexho
 * @date 2021/5/27 8:42 下午
 */
public class BasicParser {
    HashSet<String> reserved = new HashSet<String>();
    Parser.Operators operators = new Parser.Operators();
    Parser expr0 = Parser.rule();
    public Parser primary = Parser.rule(PrimaryExpr.class).or(Parser.rule().sep("(").ast(expr0).sep(")"),
            Parser.rule().number(NumberLiteral.class), Parser.rule().identifier(Name.class, reserved),
            Parser.rule().string(StringLiteral.class));
    public Parser factor = Parser.rule().or(Parser.rule(NegativeExpr.class).sep("-").ast(primary), primary);
    public Parser expr = expr0.expression(BinaryExpr.class, factor, operators);
    Parser statement0 = Parser.rule();
    public Parser block = Parser.rule(BlockStmnt.class).sep("{").option(statement0)
            .repeat(Parser.rule().sep(";", Token.EOL).option(statement0)).sep("}");
    public Parser simple = Parser.rule(PrimaryExpr.class).ast(expr);
    public Parser statement = statement0.or(
            Parser.rule(IfStmnt.class).sep("if").ast(expr).repeat(Parser.rule().sep(Token.EOL)).ast(block).option(Parser.rule().repeat(Parser.rule().sep(Token.EOL)).sep("else").ast(block)),
            Parser.rule(WhileStmnt.class).sep("while").ast(expr).ast(block), simple);
    public Parser program = Parser.rule().or(statement, Parser.rule(NullStmnt.class)).sep(";", Token.EOL);
    public Parser test = Parser.rule().or(Parser.rule().number().number().sep(Token.EOL), Parser.rule().number().number().sep(Token.EOL));

    public BasicParser() {
        reserved.add(";");
        reserved.add("}");
        reserved.add(Token.EOL);
        operators.add("=", 1, Parser.Operators.RIGHT);
        operators.add("==", 2, Parser.Operators.LEFT);
        operators.add(">", 2, Parser.Operators.LEFT);
        operators.add("<", 2, Parser.Operators.LEFT);
        operators.add("+", 3, Parser.Operators.LEFT);
        operators.add("-", 3, Parser.Operators.LEFT);
        operators.add("*", 4, Parser.Operators.LEFT);
        operators.add("/", 4, Parser.Operators.LEFT);
        operators.add("%", 4, Parser.Operators.LEFT);
    }

    public ASTree parse(Lexer lexer) throws ParseException {
        return program.parse(lexer);
    }
}
