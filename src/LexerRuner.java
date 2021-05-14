import exception.ParseException;
import token.Token;

/**
 * @author Zexho
 * @date 2021/5/14 5:28 下午
 */
public class LexerRuner {
    public static void main(String[] args) throws ParseException {
        CodeDialog codeDialog = new CodeDialog();
        Lexer l = new Lexer(codeDialog);
        for (Token t; (t = l.read()) != Token.EOF; ) {
            System.out.println("=> " + t.getText());
        }
    }
}
