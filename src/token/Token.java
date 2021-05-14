package token;

import exception.ZexException;

/**
 * @author Zexho
 * @date 2021/5/14 4:12 下午
 */
public abstract class Token {
    /**
     * end of file
     */
    public static final Token EOF = new Token(-1) {
    };

    /**
     * end of line
     */
    public static final String EOL = "\\n";
    private final int lineNumber;

    protected Token(int line) {
        lineNumber = line;
    }

    public int getLineNumber() {
        return this.lineNumber;
    }

    public boolean isIdentifier() {
        return false;
    }

    public boolean isNumber() {
        return false;
    }

    public boolean isString() {
        return false;
    }

    public int getNumber() {
        throw new ZexException("not number token");
    }

    public String getText() {
        return "";
    }

}
