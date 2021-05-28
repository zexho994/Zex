import ast.ASTree;
import exception.ParseException;
import lexer.Lexer;
import parse.BasicParser;
import token.Token;

/**
 * @author Zexho
 * @date 2021/5/14 5:28 下午
 */
public class LexerRunner {
    public static void main(String[] args) throws ParseException {
        CodeDialog codeDialog = new CodeDialog();
        Lexer l = new Lexer(codeDialog);
        BasicParser bp = new BasicParser();
        while (l.peek(0) != Token.EOF) {
            ASTree ast = bp.parse(l);
            System.out.println("program: \n" + ast);
        }
    }
}
