package lexer;

import exception.ParseException;
import token.IdToken;
import token.NumToken;
import token.StrToken;
import token.Token;

import java.io.IOException;
import java.io.LineNumberReader;
import java.io.Reader;
import java.util.ArrayList;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

/**
 * @author Zexho
 * @date 2021/5/14 4:12 下午
 */
public class Lexer {

    public static String regexPat = "\\s*((//.*)|([0-9]+)|(\"(\\\\\"|\\\\\\\\|\\\\n|[^\"])*\")|[A-Z_a-z][A-Z_a-z0-9]*|==|<=|>=|&&|\\|\\||\\p{Punct})?";
    private final Pattern pattern = Pattern.compile(regexPat);
    private final ArrayList<Token> queue = new ArrayList<>();
    private boolean hasMore;
    private final LineNumberReader reader;

    public Lexer(Reader r) {
        hasMore = true;
        reader = new LineNumberReader(r);
    }

    public Token read() throws ParseException {
        if (fillQueue(0)) {
            return queue.remove(0);
        }
        return Token.EOF;
    }

    public Token peek(int i) throws ParseException {
        if (fillQueue(i)) {
            return queue.get(i);
        }
        return Token.EOF;
    }

    public boolean fillQueue(int i) throws ParseException {
        while (i >= queue.size()) {
            if (!hasMore) {
                return false;
            }
            readLine();
        }
        return true;
    }

    public void readLine() throws ParseException {
        String line;
        try {
            line = reader.readLine();
        } catch (IOException e) {
            throw new ParseException(e);
        }
        if (line == null) {
            hasMore = false;
            return;
        }
        int lineNo = reader.getLineNumber();
        Matcher matcher = pattern.matcher(line);
        matcher.useAnchoringBounds(true).useAnchoringBounds(false);
        int pos = 0;
        int endPos = line.length();
        while (pos < endPos) {
            matcher.region(pos, endPos);
            if (matcher.lookingAt()) {
                addToken(lineNo, matcher);
                pos = matcher.end();
            } else {
                throw new ParseException("bad token at line" + lineNo);
            }
            queue.add(new IdToken(lineNo, Token.EOL));
        }
    }

    protected void addToken(int lineNo, Matcher matcher) {
        String m = matcher.group(1);
        if (m != null && matcher.group(2) == null) {
            Token token;
            if (matcher.group(3) != null) {
                token = new NumToken(lineNo, Integer.parseInt(m));
            } else if (matcher.group(4) != null) {
                token = new StrToken(lineNo, toStringLiteral(m));
            } else {
                token = new IdToken(lineNo, m);
            }
            queue.add(token);
        }
    }

    protected String toStringLiteral(String s) {
        StringBuilder sb = new StringBuilder();
        int len = s.length() - 1;
        for (int i = 1; i < len; i++) {
            char c = s.charAt(i);
            if (c == '\\' && i + 1 < len) {
                int c2 = s.charAt(i + 1);
                if (c2 == '"' || c2 == '\\') {
                    c = s.charAt(++i);
                } else if (c2 == 'n') {
                    ++i;
                    c = '\n';
                }
            }
            sb.append(c);
        }
        return sb.toString();
    }
}
