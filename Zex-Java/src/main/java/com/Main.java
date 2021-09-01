package com;

import org.antlr.v4.runtime.ANTLRInputStream;
import org.antlr.v4.runtime.CommonTokenStream;

/**
 * @author Zexho
 * @date 2021/8/31 7:32 下午
 */
public class Main {

    public static void main(String[] args) {
        String[] text = new String[]{
                "var a = 1",
                "var b = 2",
                "print a",
                "print b",
        };
        for (String s : text) {
            ANTLRInputStream in = new ANTLRInputStream(s);
            ZexLexer lexer = new ZexLexer(in);
            CommonTokenStream tokens = new CommonTokenStream(lexer);
            ZexParser parser = new ZexParser(tokens);
            parser.compilationUnit();
        }
    }

}
